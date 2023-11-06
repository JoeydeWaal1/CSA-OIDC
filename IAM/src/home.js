import { useState } from "react"

export default function Home(){
    const [user, setUser] = useState(JSON.parse(sessionStorage.getItem("user")) || {})
    console.log(user);
    console.log(user.picture)
    return (
    <>
        <img src={user.picture}></img>
        <h1 className="text-3xl">{user.name}</h1>
        <h1>{user.email}</h1>
    </>)
}