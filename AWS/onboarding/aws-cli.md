# AWS CLI

## Install AWS CLI 
[Instructions](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) to install AWS cli.

E.g. Debian-based system
```
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install
```

## Reference
Visit [AWS CLI reference](https://docs.aws.amazon.com/cli/latest/reference/) for details.

To have access to our account from the aws-cli as a best practice, we are going to create a new IAM user which will have access.

---

## 1. Create an IAM User to have CLI access
1. Login to AWS
2. Search for `IAM`
3. In `User Managment`
4. `Users`
5. `Create User`
6. Give username
7. `Next`
8. Permissions options set to `Attach policies directly`
9. Give the permissions that you want
10. Create User

To use this user from aws-cli we need access keys. Hence, click on the user, to create the access keys.

## 2. Create Access Keys
Set `Use case` to `Command Line Interface`

**DO NOT REVIEL ACCESS KEY AND SECRET ACCESS KEY!!!**

## 3. AWS-CLI Configuration
To configure the cli run
```
$ aws configure
AWS Access Key ID [None]:                           
AWS Secret Access Key [None]:  
Default region name [None]: us-east-1                                   
Default output format [None]: json                                      
```

Then, the input will be stored in `~/.aws/credentials` and `~/.aws/config` as plaintext.

## 4. Verification
To verify that aws-cli is linked with your account, run.
`aws sts get-caller-identity`

