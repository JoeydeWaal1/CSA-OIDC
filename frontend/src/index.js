import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import Callback from "./callback"
import Login from './login';
import Home from './home';

const router = createBrowserRouter([
  {
    path: "/",
    element: <App/>,
  },
  {
    path: "/callback/*",
    element: <Callback/>
  },
  {
    path: "/login",
    element: <Login/>
  },{
    path: "/home",
    element: <Home/>
  }
]);

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
     <RouterProvider router={router} />
);