# Decentralized role-based encryption (DRBE)

## Problem

The team set out to get feedback from members of a DAO community on the problems that they are facing and what they find lacking in the current system.
A response that the team received commented on the communication between the DAO’s counsel and the developers regarding the decisions that the counsel is making.
By contrast, the DAO does not want to ‘leak’ business operations to competitors or have them readily available for anyone to browse, as is the nature of putting documentation on the public blockchain.

## Concept

The solution for the problem is a system where employees can partially encrypt documents based on roles similar to redacting done in governments.
This encryption is done with the blockchain, so documents can always be decrypted by the appropriate roles.
Finally, the employee will publish the document publicly on a file storage solution in the organization.
This means that information is properly distributed based on stake or role within a DAO or organization, for example a developer can see what decisions have been made by the role above them in the hierarchy (e.g., managers, project lead, etc.).

To accomplish this, an interface must be implemented which keeps track of documents, including messages.
It is important that the authenticity and history of these documents can be validated using the blockchain.
This way, a system can be created, which only grants access to users with the appropriate roles (target role and the role under that).
This results in better transparency without risking leakage of critical business information. 