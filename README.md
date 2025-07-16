# ChainPilot: Autonomous Crypto Workflows, Piloted by Rust

ChainPilot is an EVM-compatible smart contract automation framework designed for decentralized triggers, gas-optimized execution, and seamless Chainlink Keepers integration. It empowers developers to build autonomous crypto workflows that react to on-chain and off-chain events without manual intervention. ChainPilot facilitates the creation of sophisticated decentralized applications (dApps) capable of self-executing complex logic based on predefined conditions, reducing operational overhead and enhancing efficiency.

This repository provides a robust and secure platform for automating smart contract interactions. ChainPilot offers a modular architecture built with Rust, enabling developers to define specific triggers that initiate automated actions. These triggers can be based on price thresholds, time intervals, event emissions, or any other observable condition. The execution logic is optimized for gas efficiency, minimizing costs associated with on-chain transactions. Furthermore, ChainPilot integrates directly with Chainlink Keepers, leveraging their decentralized network of nodes to ensure reliable and timely execution of automated tasks, even when the initiating smart contract is not actively interacted with.

By combining Rust's performance and safety guarantees with Chainlink Keepers' decentralized execution capabilities, ChainPilot delivers a dependable and cost-effective solution for automating a wide range of use cases. These include automated trading strategies, decentralized limit orders, recurring payments, rebalancing of DeFi portfolios, and much more. ChainPilot removes the need for centralized servers or manual intervention, providing a truly decentralized and autonomous approach to managing smart contract interactions. This allows developers to focus on building innovative applications without worrying about the complexities of maintaining and triggering them.

Key Features

*   **Decentralized Triggers:** Define custom triggers based on on-chain or off-chain data sources. Triggers are implemented as configurable smart contracts that monitor specific conditions. When a trigger condition is met, the corresponding execution logic is initiated via Chainlink Keepers. An example of trigger implementation might be:

    

*   **Gas-Optimized Execution:** ChainPilot utilizes efficient Rust code and gas-optimized smart contract design principles to minimize transaction costs. Techniques such as calldata optimization and state variable caching are employed to reduce gas consumption during task execution.
*   **Chainlink Keepers Integration:** Seamlessly integrates with Chainlink Keepers to ensure reliable and decentralized execution of automated tasks. Chainlink Keepers monitor trigger conditions and execute the defined logic when the conditions are met, even when the initiating contract is inactive.
*   **EVM Compatibility:** ChainPilot is fully compatible with the Ethereum Virtual Machine (EVM), allowing for easy integration with existing Ethereum-based smart contracts and tools.
*   **Modular Architecture:** The framework is designed with a modular architecture, enabling developers to easily extend and customize the functionality to meet their specific needs. This allows for plug-and-play functionality for different data sources and execution logic.
*   **Secure by Design:** Built with Rust's strong memory safety guarantees, minimizing the risk of vulnerabilities commonly found in Solidity-based smart contracts. Formal verification techniques can also be integrated to further enhance security.
*   **Event-Driven Architecture:** ChainPilot leverages an event-driven architecture to facilitate efficient and reactive automation. Smart contract events trigger specific actions, ensuring timely execution of automated tasks.

Technology Stack

*   **Rust:** The core framework is written in Rust, providing safety, performance, and concurrency. Rusts ownership and borrowing system prevents common memory errors, making it ideal for secure smart contract development.
*   **Solidity:** Smart contracts for trigger definitions and interacting with the core framework are written in Solidity. This allows for interoperability with existing Ethereum-based dApps.
*   **Chainlink Keepers:** Chainlink Keepers are used for decentralized task execution, ensuring reliable and timely execution of automated tasks.
*   **Foundry:** Foundry is used for smart contract development, testing, and deployment.
*   **Web3.js/Ethers.js:** JavaScript libraries for interacting with the Ethereum blockchain from client-side applications.

Installation

1.  **Install Rust:** If you don't have Rust installed, download and install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2.  **Install Foundry:** Follow the instructions at [https://book.getfoundry.sh/](https://book.getfoundry.sh/) to install Foundry.

3.  **Clone the repository:**
    git clone https://github.com/jjfhwang/ChainPilot.git
    cd ChainPilot

4.  **Install dependencies:**
    forge install

5.  **Compile smart contracts:**
    forge build

6.  **Run tests:**
    forge test

Configuration

ChainPilot relies on environment variables for configuration. Create a `.env` file in the root directory of the project.

*   `CHAINLINK_KEEPER_ADDRESS`: The address of the Chainlink Keeper contract.
*   `PRIVATE_KEY`: Your Ethereum private key. (Use a test key for development).
*   `RPC_URL`: The URL of your Ethereum node (e.g., Infura, Alchemy).

Example `.env` file:

CHAINLINK_KEEPER_ADDRESS=0x...
PRIVATE_KEY=0x...
RPC_URL=https://eth-goerli.g.alchemy.com/v2/...

Usage

1.  **Deploy the ChainPilot contracts:** Use Foundry to deploy the core ChainPilot smart contracts to your desired Ethereum network.

    forge create src/ChainPilot.sol:ChainPilot --private-key $PRIVATE_KEY --rpc-url $RPC_URL

2.  **Define your custom trigger contract:** Create a Solidity smart contract that defines the conditions for your trigger. Implement the necessary interface to interact with the ChainPilot contract.

3.  **Register your trigger with ChainPilot:** Call the `registerTrigger` function on the ChainPilot contract, providing the address of your trigger contract and any necessary parameters.

4.  **Implement your execution logic:** Create a smart contract that contains the execution logic to be triggered when the trigger conditions are met.

5.  **Link trigger and execution logic**: In the trigger contract, define a call to the execution logic contract when conditions are met. This call will be executed by Chainlink Keepers.

Example:

    // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.0;

    import "src/ChainPilot.sol";

    contract MyTrigger {
        ChainPilot public chainPilot;
        address public executionContract;

        constructor(address _chainPilot, address _executionContract) {
            chainPilot = ChainPilot(_chainPilot);
            executionContract = _executionContract;
        }

        function checkUpkeep(bytes memory) public view returns (bool upkeepNeeded, bytes memory performData) {
            // Example: check if price exceeds a threshold
            uint256 currentPrice = getPriceFromOracle(); // Replace with actual price retrieval
            uint256 threshold = 1000;
            if (currentPrice > threshold) {
                upkeepNeeded = true;
                performData = abi.encode(executionContract);
            } else {
                upkeepNeeded = false;
            }
        }

        function performUpkeep(bytes calldata performData) external {
            require(msg.sender == address(chainPilot), "Only ChainPilot can perform upkeep");
            (address _executionContract) = abi.decode(performData, (address));
            // Call the execution logic contract
            (bool success, ) = _executionContract.call(abi.encodeWithSignature("execute()"));
            require(success, "Execution failed");
        }

        function getPriceFromOracle() public view returns (uint256) {
            // Placeholder for price retrieval logic (e.g., Chainlink price feed)
            return 1200;
        }
    }

Contributing

We welcome contributions to ChainPilot! Please follow these guidelines:

1.  Fork the repository.
2.  Create a branch for your feature or bug fix.
3.  Write clear and concise code with thorough comments.
4.  Write unit tests for your changes.
5.  Submit a pull request with a detailed description of your changes.

License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/jjfhwang/ChainPilot/blob/main/LICENSE) file for details.

Acknowledgements

We would like to thank the Chainlink team for their support and for providing the essential infrastructure for decentralized task execution.