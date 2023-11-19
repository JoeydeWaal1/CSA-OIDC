import { useEffect, useState } from "react"
import axios from "axios";

const WEBSITE_HOST = "localhost" // process.env.WEBSITE_HOST;


export default function Home() {
    const [user, _setUser] = useState(JSON.parse(sessionStorage.getItem("user")) || {})
    const [todos, setTodos] = useState(null);
    const [newTodo, setNewTodo] = useState("");
    const access_token = user.access_token;

    const AddTodos = (r) => setTodos(r.data);
    const UpdateTodos = async () => {await add_todo(newTodo, access_token)}
    

    useEffect(() => {
        fetch(access_token).then(AddTodos)
    }, []);
    return (
        <>
            <img src={user.picture}></img>

            <input type="text" value={newTodo} onChange={(e)=> setNewTodo(e.target.value)}/>
            <button onClick={async () => {
                 await UpdateTodos();
                 fetch(access_token).then(AddTodos)
            }}>add</button>
            <button onClick={async () => {
                await delete_todo(access_token);
                fetch(access_token).then(AddTodos)

            }}>verwijder mijn todo's (alleen voor niet studenten)</button>
            <h1 className="text-3xl">{user.name}</h1>
            {todos?.map((t, i) => <div key={i}>{t}</div>)}
            {todos == null && <div>Loading todos</div>}
            {todos?.length === 0 && <div>nog geen todos</div>}
        </>)
}

async function add_todo(todo, access_token){
    let response = await axios.post(`http://${WEBSITE_HOST}:8080/todos`, {todo}, {
        headers: {
            "Authorization": `Bearer ${access_token}`
        },
    });
    return response
}
async function fetch(access_token) {
    let response = await axios.get(`http://${WEBSITE_HOST}:8080/todos`, {
        headers: {
            "Authorization": `Bearer ${access_token}`
        }
    });
    return response
}
async function delete_todo(access_token) {
    try {
        let response = await axios.delete(`http://${WEBSITE_HOST}:8080/todos`, {
            headers: {
                "Authorization": `Bearer ${access_token}`
            }
        });
    } catch (e) {
        console.log(e);
        alert("geen rechten!")
    }
    
}