//import { OidcClient } from "oidc-client-ts";
import { OidcClient } from "oidc-client-ts";
import { Oidcclient } from "./settings";
import { settings } from "./settings";
import { Link } from "react-router-dom";
import { useEffect } from "react";

export default function Login(){

    var client = Oidcclient;

    function signin() {
        new OidcClient(settings).createSigninRequest({}).then(function(req) {
            window.location = req.url;
        }).catch(function(err) {
            console.error(err);
        });
    }
    useEffect(() => {
        signin();
    }, [])
    return <>
    <Link to="/">Home</Link>
    <div>
    logging in
    </div>
    </>
}