# Offer credential

This simple command will result in a credential offer for a mocked credential that can be accepted immediately by scanning the generated QR code or using the URL.&#x20;

```
aries-cli automate credential-offer
```

First, you will receive a connection invitation in your wallet, after which you will receive the offered credential. The credential is named default and contains several fields and types that will help during development.&#x20;

```json
{
    "Bank": "qBank New York",
    "Name": "Joyce Brown",
    "Valid Until": "20251212",
    "Card Number": "4537-6696-0666-0146",
    "City": "New York",
    "Date Of Birth": "19890321",
    "Street": "Main Road 207",
    "Security Code": "063",
}

```

