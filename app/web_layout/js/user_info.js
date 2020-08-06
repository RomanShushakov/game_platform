"use strict";

let is_logged_in = true;

// *** identification start
async function identify_user() {
    const token = localStorage.getItem("authorization");
    if (!token) {
        is_logged_in = false;
        return {"user_name": "Guest"};
    }
    else
    {
        const url = "/auth/identify_user";

        const my_settings = {
            method: "GET",
            headers: {"authorization": token}
        };

        const user_data = await fetch(url, my_settings)
            .then(response => {
                if (response.ok) {
                    return response.text()
                    .then(text => {
                        is_logged_in = true;
                        console.log(is_logged_in);
                        console.log(JSON.parse(text));
                        return JSON.parse(text);
                    })
                }
                else {
                    return response.text()
                    .then(text => {
                        is_logged_in = false;
                        console.log(is_logged_in);
                        console.log(text);
                        return {"user_name": "Guest", "error_msg": text};
                    })
                }
            });
        return user_data; 
    }
}


window.onload = function () {
    identify_user()
    .then(user_data => {
        add_startup_greeting(user_data["user_name"]); 
        if (is_logged_in) {
            const sign_out_button = document.getElementById("sign_in_sign_out_button");
            sign_out_button.innerHTML = "Sign out";
        }
        else {
            window.location = "../";
        }
    });
};


function add_startup_greeting(user_name) {
    const greeting = document.getElementById("greeting");
    greeting.innerHTML = `Hello ${user_name}!`;
}


function add_greeting(user_name) {
    const greeting = document.createElement("h3");
    greeting.id = "greeting";
    greeting.textContent = `Hello ${user_name}!`;
    reloading_data.appendChild(greeting);
}
// *** identification end


// *** sign out start
const sign_in_sign_out_button = document.getElementById("sign_in_sign_out_button");
sign_in_sign_out_button.addEventListener("click", function () {
    if (!is_logged_in) {
        window.location = "../";
    }
    else {
        localStorage.removeItem("authorization");
        is_logged_in = false;
        window.location = "../";
    }
});
// *** sign out end
