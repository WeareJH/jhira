#!/bin/bash
#set -x #uncomment for debug mode

clear_term() {
	#comment out for output
	clear
	#echo "debug -- clear"
}

aborting_install() {
	clear_term
	echo "Aborting Install"
}

path_check() {
	path_check_var=$(which jhira)
	if [ -x "$path_check_var" ] ; then
			echo "jhira appears to be in your path! Continuing..."
			sleep 2
	else
			echo "jhira does not appear to be in your path,"
			echo "Would you like the installer to try to auto add jhira to your path? (y/n) "
			read addToPath
			if [[ $addToPath == "y" ]] ; then
					touch ~/.zshrc
					echo "export PATH=\"$PATH:/opt\"" >> ~/.zshrc
					echo "Now we are going to refresh your terminal"
					sleep 2
					source ~/.zshrc
			else
					echo "Skipping add to path step"
			fi
	fi
}

exec_install() {
	clear_term
	echo "Beginning express download of jhira"
	curl -L "https://github.com/wearejh/jhira/releases/download/v0.1.4/jhira" --output jhira-temp-binary-file
	chmod +x ./jhira-temp-binary-file
	echo "Download successful!"
	echo "You may now be asked for your password to install the jhira binary"
	sudo mkdir -p /opt
	sudo chown -R $(whoami) /opt
	mv ./jhira-temp-binary-file /opt/jhira
	path_check
	echo "Now the self-update function will run to get the latest version!"
	sleep 2
	jhira self-update
	echo "Thank you for using the express installer!"
	echo "(You may need to run \"source ~/.zshrc\" - without the quotes - to see jhira)"
}

clear_term
echo "Welcome to the jhira Express installer!"
echo "Would you like to install jhira? (y/n) "
read continueInstall
if [[ $continueInstall == "y" ]] ; then
		echo "Ok, installing now"
		path_to_executable=$(which jhira)
		if [ -x "$path_to_executable" ] ; then
				clear_term
				echo "Looks like jhira is already installed"
				echo "Would you like to reinstall?"
			    echo " - this will delete your existing installation (y/n) "
				read reinstall
				if [[ $reinstall == "y" ]] ; then
						rm $(which jhira)
						exec_install
				else
						aborting_install
				fi
		else
				exec_install
		fi
else
		aborting_install
fi

