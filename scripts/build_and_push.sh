# docker build . -t tantakle:0.0.1 --no-cache
docker image save tentakle:0.0.1 -o /tmp/tentakle.tar 
echo "[+] Pushing docker image" && scp /tmp/tentakle.tar root@192.168.46.101:/tmp/tentakle.tar
echo "[+] Loading docker image" && ssh root@192.168.46.101 "docker image load -i /tmp/tentakle.tar"
echo "[+] Stoppin docker" && ssh root@192.168.46.101 "docker stop tentakle"
echo "[+] Killing tty1" && ssh root@192.168.46.101 "pkill -t tty1"
