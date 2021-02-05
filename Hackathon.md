# Fluence ETHDenver Hackathon

## Overview
Fluence is an open application platform powered by a peer-to-peer computing protocol and a decentralized licensing system. Fluence enables developers to host applications on the decentralized network and collaborate on live applications, reusing components and data. The Fluence platform allows developers to create services at various levels of granularity, akin to microservices, and to deploy said services to a decentralized, peer-to-peer platform with Fluence's Aquamarine -- a distributed choreography language and  platform. As a result, the Fluence stack allows the creation, composition, and deployment of various reusable services into increasingly complex applications with a simplified "compose and forget" workflow. Moreover, the stack is soon to be complemented with an on-chain licensing system allowing developers and projects to monetize their IP.  
Due to its distributedness as well as source-code and workflow auditability, the Fluence stack allows web3 developers to efficiently create highly available, trusted dApps and other off-chain applications.  


## Bounty 1: Fluence Web3 Application Track
The Fluence Web3 Application track invites teams to buidl dApps and other off-chain applications, such as oracle or relay services, on the Fluence stack via integration with permissionless blockchains, such as Ethereum. For example, a Fluence-based oracle application might be comprised of 
* services pulling price (pair) data and possibly other pertinent data such as volume from and for various DEXs
* services implementing one or more data processing algorithms, e.g., drop min, max, average the remaining inputs to arrive at the "oracle"
* compose these services into an application allowing the price feed recipient to select DEX inputs and data processing algorithm
* as a bonus, teams may want to consider implementing some notion of trust scores for their application's dependency graph. Fluence's [TrustGraph](https://github.com/fluencelabs/trust-graph) implementation for the peer-to-peer network nodes may provide some guidance.  

This track carries a total bounty of USD 3,000 with the final bounty allocation at the discretion of the judges.  

## Bounty 2: Fluence Web3 Enabler Track
The Fluence Web3 Enabler Track is geared toward seeding the Fluence Web3 ecosystem with the fundamental, reusable web3 tools, as services, to accelerate future web3 development with the Fluence stack. In the Ethereum ecosystem, for example, [ethereum-rs](https://crates.io/crates/ethereum) offers a proven [break down](https://crates.io/crates/ethereum) of functional deliverables by crate as does [ethers](https://github.com/gakonst/ethers-rs). We are currently looking for contributions to enable support for Ethereum and Polkadot.  

This track carries a total bounty of USD 3,000 with the final bounty allocation at the discretion of the judges.  


The total bounty for both tracks is USD 6,000 and final bounty allocations are at the discretion of the judges.  

## Resources
* [Fluence Home](https://fluence.network/)  
* [Fluence Manifesto](https://fluence.network/manifesto.html)  
* [Fluence Protocol](https://github.com/fluencelabs/rfcs/blob/main/0-overview.md)  
* [Fluence Resources](https://fluence-labs.readme.io/docs/resources)
* [Fluence Developer Hub](https://dash.fluence.dev/)
* [Fluence Github](https://github.com/fluencelabs)  
* [Fluence Youtube channel](https://www.youtube.com/channel/UC3b5eFyKRFlEMwSJ1BTjpbw)
    * [Open applications infrastructure. Intro to Fluence](https://youtu.be/FpmT2w0zNE0)
    * [Building a front-end for a p2p app](https://youtu.be/c1WPIE5RwL4)
    * [Aquamarine 101](https://youtu.be/EcS0jT8a_dk)
    * [Create and distribute a service with Fluence Network](https://youtu.be/XmkuJkPaFEQ)
    * [Aquamarine under the hood](https://youtu.be/SQXq7Voky1w)
* [Fluence Annual DWeb report](https://medium.com/fluence-network/decentralized-web-developer-report-2020-5b41a8d86789)
* Fluence Examples and PoCs
    * [Hackathon Info And Examples](https://github.com/fluencelabs/ethdenver-hackathon)  
    * [Collabortion Example - Chat](https://github.com/fluencelabs/aqua-demo)
    * [Collabortion Example - Fluent Pad](https://github.com/fluencelabs/fluent-pad)

## Requirements
To qualify for the Fluence bounty, your project
* needs to open an [issue](https://github.com/fluencelabs/ethdenver-hackathon) with a brief
  * project description
  * track  
  * team composition
* update the issue with a link to the project repo before the ETHDenver hackathon's deadline date
* must be open-sourced, preferably with a MIT/Apache 2.0 license
* should create at least two composable services on the Fluence testnetwork and demonstrate their combined functionality and all front-end applications need to be available in docker containers  
* needs to be well documented. The documentation should include goals and purpose, instructions fo use and testing, briefly summarize challenges encountered and, if necessary, specify non-functional aspects of the project.
* if your team pushed PRs to any of the Fluence projects, list them in a dedicated section your project documentation.  


#### Judging Criteria:  
* Originality of the use case
* Potential for reuse of the submitted servicess 
* Usefulness and functionality of the submission