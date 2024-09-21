# Dukatia
This project was created to manage my own finances. Hopefully it is useful for you as well!  
You can read more on the [Dukatia website](https://dukatia.com).



# Features
- Transactions: Log your transactions and add metadata like recipients, accounts, assets and tags to get better insights into your spending habits.
- Budgets: Set yourself a budget to plan how you want to spend your money.
- Assets: Track your financial assets to get an overview over your investments.
- Multiple Accounts: Track your spending across all of your accounts.
- Powerful Tagging: Tags don't just stand on their own - they can also have a parent. This lets you define a tree-like structure for all your tagging needs.
- Multiple Currencies: You have a bank account in a foreign currency? No problem! You can also define your own custom currencies.
- Customizable Dashboards: With the Dashboard editor you can build the Dashboards you want. This allows you to get a quick overview over the data you need.
- Self-hosted & Open-Source: Your financial data tells a lot about you, so keep it safe! Dukatia is fully self-hosted and doesn't send your sensitive data to the cloud.

# Demo
You can access the live demo [here](https://demo_account_creator.dukatia.com/). This allows you to try Dukatia without having to install anything. Please don't enter personal information. The demo environment gets reset and updated once every hour. 

# Installation
The only supported way to install and run dukatia is Docker. For convenience I provide a [docker-compose.yml](https://github.com/T-x-T/dukatia/blob/main/docker-compose.yml) file that makes it really easy to get started.
For more instructions, please check out the [docs](https://dukatia.com/docs#installation).

# Configuration
Configuration is handled through envoirnment variables.
Important ones to change are:

`POSTGRES_PASSWORD` this needs to be the same for the backend and postgres services  
`ADMIN_PASSWORD` this will be the password you log in with. Right now you can only use the admin account, however multi-account support will be added later on. This variable is only relevant on first boot. Afterwards you can change it through the settings menu.  
`PEPPER` this is somewhat important for securily encrypting passwords for the database. Just roll your face across your keyboard.

You can find more info about all the configuration options in the [docs](https://dukatia.com/docs/admin/configuration). 