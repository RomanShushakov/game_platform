"use strict";

const reloading_data = document.getElementById("reloading_data");
let is_logged_in = false;

// *** identification start
async function identify_user() {
    const token = localStorage.getItem("authorization");
    is_logged_in = false;
    if (!token) {
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
                        // console.log(is_logged_in);
                        // console.log(JSON.parse(text));
                        return JSON.parse(text);
                    })
                }
                else {
                    return response.text()
                    .then(text => {
                        is_logged_in = false;
                        // console.log(is_logged_in);
                        // console.log(text);
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
            document.getElementById("register_button").remove();
        }
        const user_info_button = document.getElementById("user_info_button");
        user_info_button.innerHTML = user_data["user_name"];
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


// *** registration start
const register_button = document.getElementById("register_button");
register_button.addEventListener("click", function () {

    reloading_data.innerHTML = "";

    const registration_header =  document.createElement("h3");
    registration_header.textContent = "Register";
    reloading_data.appendChild(registration_header);
    
    const registration_user_name = document.createElement("input");
    registration_user_name.id = "registration_user_name";
    registration_user_name.placeholder = "user name";
    reloading_data.appendChild(registration_user_name);

    const registration_email = document.createElement("input");
    registration_email.id = "registration_email";
    registration_email.placeholder = "email";
    reloading_data.appendChild(registration_email);

    const registration_password = document.createElement("input");
    registration_password.id = "registration_password";
    registration_password.placeholder = "password";
    registration_password.type = "password";
    reloading_data.appendChild(registration_password);

    const register_submit_button = document.createElement("button");
    register_submit_button.id = "register_submit_button";
    register_submit_button.innerHTML = "Submit";
    reloading_data.appendChild(register_submit_button);

    const register_response_message = document.createElement("p");
    register_response_message.id = "register_response_message";
    reloading_data.appendChild(register_response_message);

    identify_user()
    .then(user_data => {
        add_greeting(user_data["user_name"]);
        const user_info_button = document.getElementById("user_info_button");
        user_info_button.innerHTML = user_data["user_name"];
    });

    register_submit_button.addEventListener("click", function () {
        const user_data = {
            "user_name": registration_user_name.value,
            "email": registration_email.value,
            "password": registration_password.value
        }; 
        if (!check_registration_data(user_data)) {
            return false;
        }
        else {
            register_user(user_data);
        }
    });
});


function register_user(user_data) {
    const url = "/auth/register_user";
    const my_settings = {
        method: "POST",
        headers: {"Content-Type": "application/json",},
        body: JSON.stringify(user_data)
    };
    fetch(url, my_settings)
        .then(response => {
            if (response.ok) {
                return response.text()
                    .then(text => {
                        reloading_data.innerHTML = "";
                        const registration_successfull_header = `<h3>${text}</h3>`;
                        reloading_data.innerHTML = registration_successfull_header;
                        const sign_in_button = document.createElement("button");
                        sign_in_button.id = "sign_in_button";
                        sign_in_button.innerHTML = "Sign in";
                        reloading_data.appendChild(sign_in_button);
                        sign_in_button.addEventListener("click", function () {
                            sign_in_form_uploading();
                        });
                    });
            }
            else {
                // return response.json()
                return response.text()
                    .then(text => {
                        register_response_message.textContent = text;
                    });
            }
        });
};
// *** registration end


// *** sign in start
const sign_in_sign_out_button = document.getElementById("sign_in_sign_out_button");
sign_in_sign_out_button.addEventListener("click", function () {
    if (!is_logged_in) {
        sign_in_form_uploading();
    }
    else {
        localStorage.removeItem("authorization");
        is_logged_in = false;
        window.location.reload();
    }
});


function sign_in_form_uploading() {

    reloading_data.innerHTML = "";

    const sign_in_header =  document.createElement("h3");
    sign_in_header.textContent = "Sign in";
    reloading_data.appendChild(sign_in_header);

    const sign_in_user_name = document.createElement("input");
    sign_in_user_name.id = "sign_in_user_name";
    sign_in_user_name.placeholder = "user name";
    reloading_data.appendChild(sign_in_user_name);

    const sign_in_password = document.createElement("input");
    sign_in_password.id = "sign_in_password";
    sign_in_password.type = "password";
    sign_in_password.placeholder = "password";
    reloading_data.appendChild(sign_in_password);

    const login_button = document.createElement("button");
    login_button.id = "login_button";
    login_button.innerHTML = "Login";
    reloading_data.appendChild(login_button);

    const sign_in_response_message = document.createElement("p");
    sign_in_response_message.id = "sign_in_response_message";
    reloading_data.appendChild(sign_in_response_message);

    identify_user()
    .then(user_data => {
        add_greeting(user_data["user_name"]);
        const user_info_button = document.getElementById("user_info_button");
        user_info_button.innerHTML = user_data["user_name"];
     });

    login_button.addEventListener("click", function() {
        const user_data = {
            "user_name": sign_in_user_name.value,
            "password": sign_in_password.value
        };
        if (!check_sign_in_data(user_data)) {
            return false;
        }
        else {
            sign_in_user(user_data);
        }
    });
}


function sign_in_user(user_data) {
    const url = "/auth/sign_in_user";
    const my_settings = {
        method: "POST",
        headers: {"Content-Type": "application/json",},
        body: JSON.stringify(user_data)
    };
    fetch(url, my_settings)
        .then(response => {
            if (response.ok) {
                return response.text()
                    .then(text => {
                        localStorage.setItem("authorization", JSON.parse(text)["access_token"]);
                        reloading_data.innerHTML = "";
                        identify_user()
                        .then(user_data => { 
                            const sign_out_button = document.getElementById("sign_in_sign_out_button");
                            sign_out_button.innerHTML = "Sign out";
                            document.getElementById("register_button").remove();
                            add_greeting(user_data["user_name"]);
                            const user_info_button = document.getElementById("user_info_button");
                            user_info_button.innerHTML = user_data["user_name"];
                        });
                    });
            }
            else {
                return response.text()
                    .then(text => {
                        sign_in_response_message.textContent = text;
                    });
            }
        });
};
// *** sign in end


// *** check registration data start
function check_email(email) {
    const re = /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    return re.test(String(email).toLowerCase());
}

function check_registration_data(user_data) {
    let elem = null;
    for (elem in user_data) {
        if (user_data[elem] == null || user_data[elem] == "") {
            alert("Please fill all required fields");
            return false;
        }
    }
    if (!check_email(user_data.email)) {
        alert("You have entered an invalid email address");
        return false;
    }
    return true;
}
// *** check registration data end


// *** check sign in data start
function check_sign_in_data(user_data) {
    let elem = null;
    for (elem in user_data) {
        if (user_data[elem] == null || user_data[elem] == "") {
            alert("Please fill all required fields");
            return false;
        }
    }
    return true;
}
// *** check sign in data end


// *** show user data start
const user_info_button = document.getElementById("user_info_button");
user_info_button.addEventListener("click", async function() {
    if (is_logged_in) {

        window.location = "user_info.html";
    }
    // else {
    //     console.log("no info");
    // }
});
