# Introduction

Policy Language is a programming language for Decentralized Identity.

## Primary Features

* Communication over Channels

    The **send** and **receive** expressions are used for communication over channels. These commands simplify the creation of clients and servers, addressing the "asymmetry between institutions and ordinary people". See: [DIDComm Messaging Specification](https://identity.foundation/didcomm-messaging/spec/#purpose-and-scope) 


* DID Data Model

    Any valid DID Document is a valid Policy Language program. Policy Language has a text form, called PolicyML, with a syntax similar to Rust, OCAML and Robin Milner's ML language. This text format is for the ease of programming. The parsed form of PolicyML, the form which is interpreted by the Policy Language interpreter, is expressed in the DID Data Model.

* Runtime Policy Checking

    The **policy** expression provides for matching against an expression before it is executed and altering program flow accordingly. Because both data and code are expressed using the DID data model, both can be sent and received over channels. When code is received, it's evaluation can be performed within a set of policies expressed in Policy Language, providing policies without onerous user-configuration requirements. 

* Integration with DID Resolution

    Policy Programs are identified by DID. The resolved DID document is the code of the Policy program. DID URL resolution provide means to bind values to names used during program execution.

* Designed for Decentralized Identity Wallets

    Policy Language aims to be to decentralized identity wallets as JavaScript is to web browsers. Wallets which include a Policy Language interpreter provide their user's with the ability to host identity applications, protected by software defined policies. 





