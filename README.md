# Decentralized role-based encryption (DRBE)

## Problem

The team set out to collect feedback from members of a DAO community on the problems they are facing and what they find lacking in the current system. A response that the team received noted a lack of communication between the DAO's counsel and the developers regarding the legal decisions being made. The DAO, however, would prefer not to have its business operations accessible to competitors or available to the general public. This is because business operations are usually documented on the public blockchain.

## Concept

The team proposes a solution to this conflict in which specific parts of documents can be redacted based on user roles. Certain members with a certain role in the organization's hierarchy are the only ones who are able to see the redacted parts of the document. For example, a developer may be able to decrypt things for other developers and managers. This way, a system can be created which only grants access to users with the appropriate roles (the target role and the role under that). This improves transparency without risking leakage of critical business information.

The systems will utilize blockchain technology and RSA encryption and decryption. The RSA algorithm will be used to encrypt and decrypt content using a public and private key pair. Using this algorithm, each role will have its own public key and a private key pair. The public key will be stored on the blockchain, and will be publicly visible. However, it is not safe to share the private key, since this is essentially the access key to the redacted content for a specific role. A possible solution to this is to encrypt the private key of the role for each member using their public key, which would result in a collection of encrypted forms of the role's private key. Each member can use their private key to decrypt the role private key, thus allowing them access to the role. These encrypted role private keys are also considered safe and will be stored on the blockchain along with the role public key.

### Document encryption flow

![image](https://user-images.githubusercontent.com/12190745/176266560-557126e7-6a47-40aa-9078-454f0713b793.png)

The document’s encryption is performed using the role's public key, which is stored and can be retrieved from the blockchain. After that, the document is encrypted using the key and is stored in file storage within the organization.

### Document decryption flow

![image](https://user-images.githubusercontent.com/12190745/176266620-f3a47bbe-7ecf-45ab-ae7a-e36b9688e878.png)

The decrypting makes use of a user's private key. The user will retrieve the private key for his role, which is stored on the blockchain using his private key. The private key for this role will allow the person to decrypt a portion of the document, thus allowing them to read that part of the document.

