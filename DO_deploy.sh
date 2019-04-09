sudo apt-get update
sudo apt-get install build-essential git
curl https://sh.rustup.rs -sSf | sh
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh
mkdir -p replicatEdu
git clone https://github.com/replicatedu/replicatedu_lib replicatEdu/replicatedu_lib 
git clone https://github.com/replicatedu/class_register replicatEdu/class_register
git clone https://github.com/replicatedu/class_database replicatEdu/class_database
wget https://github.com/codercom/code-server/releases/download/1.691-vsc1.33.0/code-server1.691-vsc1.33.0-linux-x64.tar.gz
tar code-server1.691-vsc1.33.0-linux-x64.tar.gz
