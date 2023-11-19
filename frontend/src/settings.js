// src/settings.js
import { OidcClient } from "oidc-client-ts";

const url = window.location.origin + "/oidc-client";
const WEBSITE_HOST = "localhost" // process.env.WEBSITE_HOST;
const AUTHORITY = process.env.REACT_APP_AUTHORITY; 
const CLIENT_ID =  process.env.REACT_APP_CLIENT_ID; 
console.log(AUTHORITY);
const settings = {
    authority: `https://${AUTHORITY}`,
    client_id: CLIENT_ID,
    redirect_uri: `http://${WEBSITE_HOST}:3000/callback`,
    post_logout_redirect_uri: url + "/sample.html",
    response_type: "code",
    scope: "openid email profile",
};
const client = new OidcClient(settings);
export {settings, client}