"use strict";

const reloading_data = document.getElementById("reloading_data");

// *** identification start
function identify_user() {
    const token = localStorage.getItem("authorization");
    if (!token) {
        sessionStorage.setItem("user_name", "Guest");
        add_greeting_header();
    }
    else
    {
        const url = "/auth/identify_user";

        const my_settings = {
            method: "GET",
            headers: {"authorization": token}
        };

        fetch(url, my_settings)
            .then(function (response) {
                if (response.ok) {
                    return response.text()
                        .then(function (text) {
                            sessionStorage.setItem("user_name", JSON.parse(text)["user_name"]);
                            add_greeting_header();
                        })
                }
                else {
                    return response.text()
                        .then(function (text) {
                            console.log(text);
                            if (text == "Session has expired, please login again.") {
                                sessionStorage.setItem("user_name", "Guest");
                                add_greeting_header();
                            }
                            else {
                                sessionStorage.setItem("user_name", text);
                                add_greeting_header();
                            }
                        })
                }
            });

        
    }
}

window.onload = function () {
    identify_user();
    add_startup_greeting_header();
};

function add_startup_greeting_header() {
    const greeting_header = document.getElementById("greeting_header");
    const current_user = sessionStorage.getItem("user_name");
    greeting_header.innerHTML = `Hello ${current_user}!`;
}

function add_greeting_header() {
    const current_user = sessionStorage.getItem("user_name");
    const greeting_header = document.createElement("h3");
    greeting_header.id = "greeting_header";
    greeting_header.textContent = `Hello ${current_user}!`;
    reloading_data.appendChild(greeting_header);
}
// *** identification end


// *** registration start
const register_button = document.getElementById("register_button");
register_button.addEventListener("click", function () {

    reloading_data.innerHTML = "";

    add_greeting_header();

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
    reloading_data.appendChild(registration_password);

    const register_submit_button = document.createElement("button");
    register_submit_button.id = "register_submit_button";
    register_submit_button.innerHTML = "submit";
    reloading_data.appendChild(register_submit_button);

    const register_response_message = document.createElement("p");
    register_response_message.id = "register_response_message";
    reloading_data.appendChild(register_response_message);

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
        .then(function (response) {
            if (response.ok) {
                return response.text()
                    .then(function (text) {

                        reloading_data.innerHTML = "";
                        const registration_successfull_header = `<h3>${text}</h3>`;
                        reloading_data.innerHTML = registration_successfull_header;
                        const sign_in_button = document.createElement("button");
                        sign_in_button.id = "sign_in_button";
                        sign_in_button.innerHTML = "sign in";
                        reloading_data.appendChild(sign_in_button);

                        sign_in_button.addEventListener("click", function () {
                            sign_in_form_uploading();
                        });
                    });
            }
            else {
                // return response.json()
                return response.text()
                    .then(function (text) {
                        // throw Error(json["detail"]);
                        // console.log("Error");
                        register_response_message.textContent = text;
                    });
            }
        });
    /* .catch(function(err) {
      console.log(err);
      response_text.textContent = err;
    }); */
};
// *** registration end


// *** sign in start
const sign_in_button = document.getElementById("sign_in_button");
sign_in_button.addEventListener("click", function () {
    sign_in_form_uploading();
});

function sign_in_form_uploading() {

    reloading_data.innerHTML = "";

    identify_user();

    // add_greeting_header();

    const sign_in_header =  document.createElement("h3");
    sign_in_header.textContent = "Sign in";
    reloading_data.appendChild(sign_in_header);

    const sign_in_user_name = document.createElement("input");
    sign_in_user_name.id = "sign_in_user_name";
    sign_in_user_name.placeholder = "user name";
    reloading_data.appendChild(sign_in_user_name);

    const sign_in_password = document.createElement("input");
    sign_in_password.id = "sign_in_password";
    sign_in_password.placeholder = "password";
    reloading_data.appendChild(sign_in_password);

    const login_button = document.createElement("button");
    login_button.id = "login_button";
    login_button.innerHTML = "login";
    reloading_data.appendChild(login_button);

    const sign_in_response_message = document.createElement("p");
    sign_in_response_message.id = "sign_in_response_message";
    reloading_data.appendChild(sign_in_response_message);

    login_button.addEventListener("click", function () {

        const user_data = {
            "user_name": sign_in_user_name.value,
            "password": sign_in_password.value
        };

        sign_in_user(user_data);
        



        // reloading_data.innerHTML = "";
        // login_button.remove();
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
        .then(function (response) {
            if (response.ok) {
                return response.text()
                    .then(function (text) {

                        localStorage.setItem("authorization", JSON.parse(text)["access_token"]);

                        reloading_data.innerHTML = "";

                        identify_user();

                        // add_greeting_header();

                        // reloading_data.innerHTML = text;


                        // const registration_successfull_header = `<h3>${text}</h3>`;
                        // reloading_data.innerHTML = registration_successfull_header;
                        // const sign_in_button = document.createElement("button");
                        // sign_in_button.id = "sign_in_button";
                        // sign_in_button.innerHTML = "sign in";
                        // reloading_data.appendChild(sign_in_button);

                        // sign_in_button.addEventListener("click", function () {
                        //     sign_in_form_uploading();
                        // });
                    });
            }
            else {
                // return response.json()
                return response.text()
                    .then(function (text) {
                        // throw Error(json["detail"]);
                        // console.log("Error");

                        sign_in_response_message.textContent = text;
                    });
            }
        });
    /* .catch(function(err) {
      console.log(err);
      response_text.textContent = err;
    }); */
};


// *** sign in end


// *** check registration data
function check_email(email) {
    const re = /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    return re.test(String(email).toLowerCase());
}

function check_registration_data(user_data) {
    let elem = null;
    for (elem in user_data) {
        if (user_data[elem] == null || user_data[elem] == "") {
            alert("Please Fill All Required Field");
            return false;
        }
    }
    if (!check_email(user_data.email)) {
        alert("You have entered an invalid email address");
        return false;
    }
    return true;
}
