import subprocess
import re
import ipaddress
import datetime
import os

def get_netstat_output():
    # Run the netstat command and get its output
    result = subprocess.run(['netstat', '-an'], stdout=subprocess.PIPE)
    return result.stdout.decode('utf-8')

def extract_unique_ips(netstat_output):
    # Use regex to find all IP addresses in the netstat output
    ip_pattern = re.compile(r'\b(?:\d{1,3}\.){3}\d{1,3}\b|(?:[a-fA-F0-9]{1,4}:){7}[a-fA-F0-9]{1,4}\b')
    ips = ip_pattern.findall(netstat_output)
    # Filter out localhost IPs and ensure uniqueness
    unique_ips = set(ip for ip in ips if not ip.startswith('127.') and ip != '::1')
    return unique_ips

def filter_private_ips(ips):
    # Filter out private IP addresses
    public_ips = {ip for ip in ips if not ipaddress.ip_address(ip).is_private}
    return public_ips

def save_ips_to_file(ips):
    # Save the unique IPs to a file named with the current date and time
    timestamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
    filename = f"unique_ips_{timestamp}.txt"
    with open(filename, 'w') as file:
        for ip in ips:
            file.write(f"{ip}\n")
    print(f"Unique IPs saved to {filename}")
    # Save the name of the most recently created file
    with open("latest_file.txt", 'w') as latest_file:
        latest_file.write(filename)
    return filename

def main():
    netstat_output = get_netstat_output()
    unique_ips = extract_unique_ips(netstat_output)
    public_ips = filter_private_ips(unique_ips)
    save_ips_to_file(public_ips)

if __name__ == "__main__":
    main()
