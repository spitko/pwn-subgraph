<script lang="ts">
  import type { PageData } from "./$houdini";
  export let data: PageData;

  $: ({ Loans } = data);

  const formatDate = (timestamp: number) => {
    if (!timestamp) return "-";
    return new Date(timestamp * 1000).toLocaleString();
  };
</script>

<div class="card basis-1/2 rounded-lg border-2 m-5 overflow-x-auto">
  <div class="card-body">
    <h2 class="card-title">Loans</h2>
    <table class="table table-xs table-zebra">
      <thead>
        <tr>
          <th>Creation Date</th>
          <th>Expiration Date</th>
          <th>Status</th>
          <th>Repaid Date</th>
          <th>Loan ID</th>
          <th>Loan Asset</th>
          <th>Loan Amount</th>
          <th>Loan Yield</th>
          <th>Collateral Category</th>
          <th>Collateral Amount</th>
          <th>Token ID</th>
          <th>Lender</th>
          <th>Borrower</th>
        </tr>
      </thead>
      <tbody>
        {#each $Loans.data?.loans ?? [] as loan}
          <tr>
            <td class="whitespace-nowrap">{formatDate(loan.createdDate)}</td>
            <td class="whitespace-nowrap">{formatDate(loan.expiration)}</td>
            <td>{loan.status}</td>
            <td class="whitespace-nowrap">{formatDate(loan.repaidDate)}</td>
            <td>{loan.id}</td>
            <td>{loan.borrowAsset.token?.symbol}</td>
            <td>{loan.borrowAmount}</td>
            <td>{(loan.repayAmount - loan.borrowAmount).toFixed(2)}</td>
            <td>{loan.collateral.category}</td>
            <td>{loan.collateralAmount ?? "-"}</td>
            <td>{loan.collateralTokenId ?? "-"}</td>
            <td>{loan.lender.id}</td>
            <td>{loan.borrower.id}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
<div class="flex">
  <div class="card basis-1/2 rounded-lg border-2 m-5">
    <div class="card-body">
      <h2 class="card-title">Most borrowed</h2>
      <table class="table table-xs">
        <thead>
          <th>Account</th>
          <th>Total Borrowed</th>
          <th>Total Lended</th>
        </thead>
        <tbody>
          {#each $Loans.data?.mostBorrowed ?? [] as account}
            <tr class="hover">
              <td>{account.id}</td>
              <td>{account.totalBorrowedLoans}</td>
              <td>{account.totalLendedLoans}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
  <div class="card basis-1/2 rounded-lg border-2 m-5">
    <div class="card-body">
      <h2 class="card-title">Most Lended</h2>
      <table class="table table-xs">
        <thead>
          <th>Account</th>
          <th>Total Borrowed</th>
          <th>Total Lended</th>
        </thead>
        <tbody>
          {#each $Loans.data?.mostLended ?? [] as account}
            <tr class="hover">
              <td>{account.id}</td>
              <td>{account.totalBorrowedLoans}</td>
              <td>{account.totalLendedLoans}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>
