# AWS Certificate Manager (ACM)

## Request a Certificate 
To enable https on a service we must first create a certifate, since we are using AWS
and it is free, we can create a certificate using ACM.

1. Search `ACM Certificate Manager`
2. `Request`
3. `Request a public certificate`
4. Give your: `*<domain_name>.com` (put star at the beginning)
5. `DNS validation`
6. Create your certificate

## Domain name register
Then copy `CNAME name` & `CNAME value` from your ACM and copy those values to your domain name registry as follows. 
- For `CNAME name` remove the domain part and keep only the underscore with the random numbers.
- For `CNAME value` remove the dot at the end.

| record type | Name | Data | 
| ------------| ----| -------|
| CNAME | _\<some-random-numbers\> | _\<some-random-numbers\>.\<some-random-numbers\>.acm-validations.aws |

Evaluation:
`dig @8.8.8.8 CNAME _<some-random-numbers>.<domain> +short`

The above command should return the `CNAME value`.