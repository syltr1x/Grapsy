#!/bin/bash

function httpd_service() {
	httpd
	if [ $? == 127 ]; then
		sudo pacman -S apache
	fi
	sudo systemctl enable httpd
	sudo systemctl start httpd
}
function noip_service() {
	# Check if you have rust
	cargo
	if [ $? != 0 ]; then
		sudo pacman -S rust
	fi
	curl -s https://dmej8g5cpdyqd.cloudfront.net/downloads/noip-duc_3.3.0.tar.gz -o noip.tar.gz 
	tar -xf noip.tar.gz
	cd noip-duc*
	cargo build --release
	sudo mv target/release/noip /usr/bin/noip
	sudo chmod +x /usr/bin/noip
	sudo systemctl enable noip
	sudo systemctl start noip
}
function colorize_service() {
	local status=$1
	if [[ "$status" == "enabled" || "$status" == "active" ]]; then
		echo -e "\e[32m$status\e[0m"
	else
		echo -e "\e[31m$status\e[0m"
	fi
}
function main() {
	# Asign services variables
	httpd_status=$(systemctl is-active httpd)
	httpd_startup=$(systemctl is-enabled httpd)
	noip_status=$(systemctl is-active noip)
	noip_startup=$(systemctl is-enabled noip)
	
	# Users list
	users_list=$(awk -F: '$3 >= 1000 && $1 != "nobody" && $6 != "/" {print $1}' /etc/passwd)
	# Local Ip
	priv_ip=$(ip a show | grep 'inet ' | awk '{ print $2 }' | cut -d/ -f1 | tail -1)

	# SSH info
	if [ -d "$HOME/.ssh" ]; then
		ssh_dir="\e[32mexists\e[0m"
	else
		ssh_dir="\e[31mdoesn't exist [!]\e[0m"
	fi
	ssh_trusted=$(cat "$HOME/.ssh/authorized_keys" 2>/dev/null | wc -l)

	# Httpd info
	httpd_dirs=$(grep -oP '<Directory "\K[^"]+' /etc/httpd/conf/httpd.conf 2>/dev/null || echo "no httpd dirs configured")

	echo -e "-- Network Info --"
	echo -e "Private IP: $priv_ip"
	echo -e "\n-- Services Startup --"
	echo -e "httpd service status: $(colorize_service $httpd_startup)"
	echo -e "noip service status: $(colorize_service $noip_startup)"
	echo -e "\n-- Services Status --"
	echo -e "httpd service status: $(colorize_service $httpd_status)"
	echo -e "noip service status: $(colorize_service $noip_status)"
	echo -e "\n-- SSH Info --"
	echo -e "SSH folder: $ssh_dir"
	echo -e "Trusted devices: $ssh_trusted"
	echo -e "\n-- Httpd dirs --"
	echo -e "$httpd_dirs"
	echo -e "\n-- Server users --"
	echo -e "$users_list"
}

main
