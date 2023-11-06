import { OidcClient } from "oidc-client-ts";
import { settings } from "./settings";
import { Link, useNavigate } from "react-router-dom";
import { useEffect } from "react";

export default function Callback(){
    const navigate = useNavigate();
    useEffect(() => {
        new OidcClient(settings).processSigninResponse(window.location.href).then(function(response){
            console.log(response)
            const user = {
                email: response.profile.email || "no email",
                name: response.profile.name || "no name",
                access_token: response.access_token || "",
                id_token: response.id_token || "",
                picture: response.profile.picture || ""
            }
            sessionStorage.setItem("user", JSON.stringify(user))
            navigate("/home")
    
        }).catch(function(err) {
            console.error(err);
        });
    } ,[])
    

    return <>
        <Link to="/">Home</Link>
        <div>
        called back
        </div>
        </>
}