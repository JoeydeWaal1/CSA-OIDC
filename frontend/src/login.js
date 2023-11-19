// src/login.js
import { client } from "./settings";
import { Link } from "react-router-dom";
import { useEffect } from "react";

export default function Login() {
    function signin() {
        client.createSigninRequest({}).then(function (req) {
            window.location = req.url;
        }).catch(function (err) {
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