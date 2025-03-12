# Cloudflare

## 0. Initialization!
Before doing anything else, create an account :P.

> [!IMPORTANT]
> DO NOT FORGET TO ENABLE ALSO 2FA.

## 1. Host site from Github to Cloudflare pages
1. Connect to [dash.cloudflare.com](dash.cloudflare.com)
2. `Workers & Pages`
3. `Pages`
4. `Connect to Git`

## 2. Use custom domain
1. Connect to [dash.cloudflare.com](dash.cloudflare.com)
2. `Workers & Pages`
3. `Pages`
4. `Custom domains`
5. `Set up a custom domain`
6. Put your domain there
7. `Activate domain`
8. Choose your plan
9. Review DNS records
10. Finally update the DNS servers on your DNS provider

In your ***Cloudflare*** Account a `CNAME` record should be appeared.
| Type | Name | Content | Proxy Status | TTL |
| ---- | ---- | ------- | ------------ | --- |
| CNAME | <domain>.com | <domain>.pages.dev | Proxied | Auto

### 3. WWW -> Root domain
1. Connect to [dash.cloudflare.com](dash.cloudflare.com)
2. `Bulk Redirects`
3. `Create Buld Redirect List`
4. Give a name like `<site>_www_redirects` and add a description
5. `Or, manually add URL redirects`
6. Fill like this
   1. **Source URL**: `www.<your domain>.com`
   2. **Target URL**: `https://<your_domain>.com`
   3. **Status**: `301`
   4. **Edit parameters**:
      1. [X] `Preserve query string`
      2. [X] `Include subdomains`
      3. [X] `Subpath matching`
      4. [X] `Preserve path suffix`
   5. **Review and edit**
7. `Redirect Rules`
   1. Set Rule name like this: `<site>_www_redirects`
   2. `Save and Deploy`
8. Add an `A record` on your DNS records like this:
    | Type | Name | Content | Proxy Status | TTL |
    | ---- | ---- | ------- | ------------ | --- |
    | A | www | 192.0.2.1 | Proxied | Auto

---

## Print hosting server info
https:// domain **/cdn-cgi/trace**

## Share admin to another accout
1. Connect to [dash.cloudflare.com](dash.cloudflare.com)
2. `Manage Account`
3. `Invite`
4. `Scope` > `Type`: All domains
5. `Account Scoped Roles` > Administrator