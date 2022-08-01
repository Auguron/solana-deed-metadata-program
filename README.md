## Deed Metadata Program
Records details about an ownership transfer of some kind, storing them
as on-chain account data.

This serves two purposes:
- Publication of the information on a censorship-resistant, open-access platform,
where trusted addresses can make proclamations that anyone can view.
- By on-ramping this kind of data on-chain, it becomes readable by other on-chain
programs.

### How it Works
#### Deed Metadata Accounts
These accounts store data that describes a transfer of ownership of some given product.
They are created and managed by an authority, which is simply any signing account.
The authority can the deed metadata account at any time to retrieve the lamports,
and it can change certain fields on the account.

#### What Deed Metadata Accounts Store
- Date of Registration -- a timestamp of when the transfer took place
- Price -- total amount paid in exchange for transfer of ownership (in smallest integer unit of currency, e.g. cents).
- Price Denomination -- 
- Buyer -- A string representing the name of a buyer. Optionally an address, but the scheme does not require it.
- Seller -- A string representing the name of a seller.
- Property -- A string representing the name of some property. This could be anything, as long as it
is practically unique across the kinds of items whose ownership transfer is being recorded. This value is hashed and is part of the input to the derivation of the deed metadata account's address.
- Uri -- A string representing an online resource where more information can be stored. There is no expected schema here, and this could link JSON data, a website, a PDF, or whatever the authority desires.

#### What Determines a Deed Metadata Account's Address?
The addresses of deed metadata accounts are deterministic, and based on certain
inputs. Those inputs are:
- The address of the originating authority (whichever Solana account paid for
the creation of the metadata account).
- The hash of a unique identifier for the property. The scheme for this identifier
is arbitrary, up to the author, and naturally varies depending on the use case. For
real estate as an example, this could be the legal address. 

Taken together, a deed metadata account can be construed as a proclamation by
some _authority_, that some _property_ has changed ownership from one party to another.

While the originating authority is a frozen value, a deed metadata account's authority can be transferred. This is useful for cold storage.

### Anyone Can Say Anything, But You Don't Have to Listen
While any signer with lamports can create deed metadata accounts, their testimony is
only as good as the trustworthiness of the signing authorities creating those accounts.
This can be broken down into two questions:
1. Do I trust that this address X is actually entity Y?
2. Do I trust entity Y?

We believe that the answer to (2) is a problem most companies have to solve for themselves
already, and that (1) naturally follows from a combination of (2) and the fact that
entity Y might do something like very publicly state "Address X is ours."

This ultimately means that untrustworthy agents have no incentive to use this system, but
trustworthy entities do. For trustworthy agents, this is a cheap and powerful way to
publish open-access information, and allow others the possibility of building on top of that
information.

#### Instructions
- `NewDeedMetadata` -- Creates a new deed metadata account, and populates its data fields.
- `CloseDeedMetadata` -- Closes a deed metadata account, returning the lamports to the authority. Can only be executed by the account's authority.
- `UpdateUri` -- Updates the URI field of a deed metadata account. Can only be executed by the account's authority.
- `TransferAuthority` -- Updates the authority field to a new address. Can only be executed by the account's authority.
