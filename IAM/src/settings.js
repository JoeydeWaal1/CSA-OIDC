import { OidcClient, UserManager } from "oidc-client-ts";

const url = window.location.origin + "/oidc-client";

const settings = {
    authority: "https://dev-uzo17fcir336ze1h.us.auth0.com",
    client_id: "IKzhr3kTQ6AtnIMA0PXJptorXEcW5h8O",
    redirect_uri: "http://localhost:3000/callback",
    post_logout_redirect_uri: url + "/sample.html", // ?
    response_type: "code",
    scope: "openid email profile",
    //response_mode: "fragment", // ?
    filterProtocolClaims: true, // ?
    client_secret: "A8jL23IIPwqOWyYb0y0IZT8lFdq4euT8CXOkvjxKXkut4dcbInzspCv7C1irOwh8"
};
const Oidcclient = new UserManager(settings);
export {Oidcclient, settings}