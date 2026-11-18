---
name: azure-storage-key
apply: "*"
severity: error
pattern: "AccountKey=[A-Za-z0-9+/]{40,}={0,2}"
examples:
  bad:
    - "conn = \"DefaultEndpointsProtocol=https;AccountName=x;Account¦Key=abcdefghijklmnopqrstuvwxyz0123456789ABCD==\""
  good:
    - "conn = os.environ[\"AZURE_STORAGE_CONNECTION_STRING\"]"
---

# Clé de compte de stockage Azure en clair

Les chaînes de connexion Azure Storage contiennent un secret : utiliser Key Vault ou l'environnement.
