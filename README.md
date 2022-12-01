# Porkbun DDNS

A simple Porkbun client capable of updating the IPs of your subdomains
and download the SSL certificates associated to your domain.  
This client uses the [Porkbun v3API](https://porkbun.com/api/json/v3/documentation) to 
update the DNS record and download the certificates.

## Configurations

Inside the `config.yaml.example` you can find an example of configurations.  
The configurations file have the following field:  
* `api_key`: Api key to interact with the Porkbun API;
* `api_secret`: Api secret to interact with the Porkbun API;
* `domain`: Your domain;
* `sub_domains`: List of subdomains which the client will update the DNS record;
* `ssl_path`: Path where will be saved the SSL certificates and keys.

## Update subdomains DNS records

To update the DNS records of your subdomains run the following command:
```bash
porkbun-ddns update-domains config.yaml
```
This command will fetch you current IP and updates the DNS records of the 
subdomains specified inside the configurations file.

# Download SSL certificates

To download the SSL certificates of your domain run the following command:
```bash
porkbun-ddns get-certificates config.yaml
```
This command will download the SSL certificates of the domain provided in
the configurations file and download al the files inside the specified 
`ssl_path`.