## PWN Subgraph

This repository contains the code for PWN Subgraph. The latest version of this subgraph can be explored at https://api.studio.thegraph.com/query/25077/pwn/version/latest. There is a simple frontend that uses this subgraph at https://spitko.github.io/pwn-subgraph/.


### Example queries

#### All loans

```gql
query Loans {
  loans(orderBy: createdDate, orderDirection: desc) {
    id
    borrower {
      id
    }
    lender {
      id
    }
    expiration
    status
    collateral {
      id
      category
      token {
        name
        symbol
        decimals
      }
    }
    collateralAmount
    collateralTokenId
    borrowAsset {
      id
      category
      token {
        name
        symbol
        decimals
      }
    }
    loanPaidBackEvents {
      transactionHash
    }
    borrowAmount
    repayAmount
    createdDate
    repaidDate
  }
}
```

#### Accounts that have borrowed the most times

```gql
{
  accounts(orderBy:totalBorrowedLoans, orderDirection:desc) {
    id
    totalLendedLoans
    totalBorrowedLoans
  }
}
```

#### Accounts with active loans at a specific block
```gql
{
  accounts(block: {number:19999284} where:{borrowed_: {status: RUNNING}}) {
    id
    borrowed {
      id
      expiration
      status
      collateral {
        id
      }
      collateralAmount
      collateralTokenId
      borrowAsset {
        id
      }
    }
    totalBorrowedLoans
  }
}
```


