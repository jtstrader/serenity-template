# Serenity Template

This is a template repository for [Serenity](https://github.com/serenity-rs/serenity), the Rust library for the Discord API. The purpose of this template is to expedite the creation of bots by providing sample code, GitHub actions, and configurations up-front. This template defaults to using [Google Cloud Platform (GCP)](https://cloud.google.com/gcp?utm_source=google&utm_medium=cpc&utm_campaign=na-US-all-en-dr-bkws-all-all-trial-e-dr-1605212&utm_content=text-ad-none-any-DEV_c-CRE_665735450627-ADGP_Hybrid+%7C+BKWS+-+EXA+%7C+Txt_Google+Cloud-KWID_43700077223807304-aud-2232802565252:kwd-6458750523&utm_term=KW_google+cloud-ST_google+cloud&gad_source=1&gclid=CjwKCAiApaarBhB7EiwAYiMwqtMwroLdLFiecWvy2IGkEZaT-esH5ILoNNXAkJhTcBBYa-lshSQ1gBoCgjMQAvD_BwE&gclsrc=aw.ds&hl=en) as the deployment target. The application is first built and then containerized before being uploaded to GCP. The deployment is triggered either manually or any push to main.

> Note that the Discord bot must have the `MESSAGE CONTENT` intent selected for the bot to work with this template out of the box.

## Secrets and Variables

This template uses multiple GitHub secrets and variables to populate fields in the CD pipeline to deploy to the Docker Hub registry and GCP.

| Secret Name             | Description                                                                                                                                   |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| DISCORD_TOKEN           | The secret for the production discord bot                                                                                                     |
| DOCKERHUB_USERNAME      | A username that has write access to the repository that will store the container                                                              |
| DOCKERHUB_TOKEN         | A valid [Docker Hub access token](https://docs.docker.com/security/for-developers/access-tokens/) that can be used to verify the user's login |
| GCP_PROJECT_ID          | The ID of the GCP project that will be used for hosting the application                                                                       |
| GCP_SERVICE_ACCOUNT_KEY | The [service account](https://cloud.google.com/iam/docs/service-accounts-create) key with permissions to create new Cloud Run instances       |

| Variable Name        | Description                                                                                 |
| -------------------- | ------------------------------------------------------------------------------------------- |
| DOCKERHUB_REPOSITORY | The name of the Docker Hub repository that the container will be stored in                  |
| GCP_SERVICE_REGION   | The [region](https://cloud.google.com/run/docs/locations) the container will be deployed in |
