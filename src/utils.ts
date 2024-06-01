import { Address, BigInt, Bytes } from "@graphprotocol/graph-ts";
import { Account, Asset, Loan, Token } from "../generated/schema";
import { ERC20 } from "../generated/PWNSimpleLoan/ERC20";
import { LOANCreated } from "../generated/PWNSimpleLoan/PWNSimpleLoan";
import { BIGINT_ONE, BIGINT_ZERO, bigIntToBigDecimal } from "./numbers";
import { AssetCategory, LoanStatus } from "./constants";

export function getOrCreateToken(id: Bytes): Token {
  let token = Token.load(id);
  if (token == null) {
    let contract = ERC20.bind(Address.fromBytes(id));
    token = new Token(id);
    token.name = contract.name();
    token.symbol = contract.symbol();
    token.decimals = contract.decimals().toI32();
    token.save();
  }
  return token;
}

export function getLoan(id: BigInt): Loan {
  let loan = Loan.load(id.toString())!;
  return loan;
}

export function createNewLoanFromEvent(event: LOANCreated): Loan {
  let loan = new Loan(event.params.loanId.toString());
  loan.createdDate = event.block.timestamp;
  const borrower = getOrCreateAccount(event.params.terms.borrower)
  borrower.totalBorrowedLoans = borrower.totalBorrowedLoans.plus(BIGINT_ONE);
  loan.borrower = borrower.id;
  borrower.save();
  const lender = getOrCreateAccount(event.params.terms.lender);
  lender.totalLendedLoans = lender.totalLendedLoans.plus(BIGINT_ONE);
  lender.save();
  loan.lender = lender.id;
  loan.expiration = event.params.terms.expiration;
  loan.collateral = getOrCreateAsset(
    event.params.terms.collateral.category,
    event.params.terms.collateral.assetAddress
  ).id;
  if (event.params.terms.collateral.amount.gt(BIGINT_ZERO)) {
    loan.collateralAmount = event.params.terms.collateral.amount;
  } else {
    loan.collateralTokenId = event.params.terms.collateral.id;
  }
  loan.borrowAsset = getOrCreateAsset(
    event.params.terms.asset.category,
    event.params.terms.asset.assetAddress
  ).id;
  const borrowToken = getOrCreateToken(event.params.terms.asset.assetAddress);
  loan.borrowAmount = bigIntToBigDecimal(
    event.params.terms.asset.amount,
    borrowToken.decimals
  );
  loan.repayAmount = bigIntToBigDecimal(
    event.params.terms.loanRepayAmount,
    borrowToken.decimals
  );
  loan.status = LoanStatus.RUNNING;
  loan.save();
  return loan;
}

export function getOrCreateAsset(category: i32, assetAddress: Bytes): Asset {
  let asset = Asset.load(assetAddress);
  if (asset == null) {
    asset = new Asset(assetAddress);
    asset.category = getAssetCategory(category);
    if (asset.category === "ERC20") {
      let token = getOrCreateToken(assetAddress);
      asset.token = token.id;
      asset.save();
    }
    asset.save();
  }
  return asset;
}

export function getOrCreateAccount(id: Bytes): Account {
  let account = Account.load(id);
  if (account == null) {
    account = new Account(id);
    account.totalBorrowedLoans = BIGINT_ZERO;
    account.totalLendedLoans = BIGINT_ZERO;
    account.save();
  }
  return account;
}

function getAssetCategory(category: i32): string {
  switch (category) {
    case 0:
      return AssetCategory.ERC20;
    case 1:
      return AssetCategory.ERC721;
    case 2:
      return AssetCategory.ERC1155;
    case 3:
      return AssetCategory.CryptoKitties;
    default:
      return "Unknown";
  }
}
