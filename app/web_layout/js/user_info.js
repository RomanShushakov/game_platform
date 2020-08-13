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

            const token = localStorage.getItem("authorization");  
            const url = "/auth/user_info";
            const my_settings = {
                method: "GET",
                headers: {"authorization": token}
            };

            fetch(url, my_settings)
                .then(response => {
                    if (response.ok) {
                        return response.text();
                    }
                    else {
                        return response.text();
                    }
                })
                .then(result => {
                    document.getElementById("user_info").innerHTML = result;
                });
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
        // console.log("user_info_logged_in");
        window.location = "../";
    }
    else {
        localStorage.removeItem("authorization");
        is_logged_in = false;
        // console.log("user_info");
        window.location = "../";
    }
});
// *** sign out end



// *** user data editing start
function edit_name() {
    const current_user_name_editing_container = document.getElementById("user_name_editing_container");
    const current_user_name_editing_field = document.createElement("input");
    current_user_name_editing_field.id = "current_user_name_editing_field";
    current_user_name_editing_container.appendChild(current_user_name_editing_field);

    const apply_changes_button = document.getElementById("apply_changes_button");

    if (!apply_changes_button) {
        create_apply_and_cancel_buttons();
    }
    document.getElementById("current_user_name_edit_button").disabled = true;
    document.getElementById("current_user_name_edit_cancel_button").disabled = false;
}


function cancel_edit_name() {
    const current_user_name_editing_container = document.getElementById("user_name_editing_container");
    current_user_name_editing_container.innerHTML = "";
    document.getElementById("current_user_name_edit_button").disabled = false;
    document.getElementById("current_user_name_edit_cancel_button").disabled = true;

    if (document.getElementById("current_password_change_cancel_button").disabled && 
        document.getElementById("current_email_edit_cancel_button").disabled && 
        document.getElementById("current_user_name_edit_cancel_button").disabled) {

        document.getElementById("apply_changes_button").remove();  
        document.getElementById("cancel_changes_button").remove();

        if (document.getElementById("user_update_response_message")) {
            document.getElementById("user_update_response_message").remove();
        }
    }
}


function edit_email() {
    const current_email_edit_container = document.getElementById("email_editing_container");
    const current_email_editing_field = document.createElement("input");
    current_email_editing_field.id = "current_email_editing_field";
    current_email_edit_container.appendChild(current_email_editing_field);

    const apply_changes_button = document.getElementById("apply_changes_button");
    if (!apply_changes_button) {
        create_apply_and_cancel_buttons();
    }
    document.getElementById("current_email_edit_button").disabled = true;
    document.getElementById("current_email_edit_cancel_button").disabled = false;
}


function cancel_edit_email() {
    const current_email_edit_container = document.getElementById("email_editing_container");
    current_email_edit_container.innerHTML = "";
    document.getElementById("current_email_edit_button").disabled = false;
    document.getElementById("current_email_edit_cancel_button").disabled = true;

    if (document.getElementById("current_password_change_cancel_button").disabled && 
        document.getElementById("current_email_edit_cancel_button").disabled && 
        document.getElementById("current_user_name_edit_cancel_button").disabled) {

        document.getElementById("apply_changes_button").remove();  
        document.getElementById("cancel_changes_button").remove();

        if (document.getElementById("user_update_response_message")) {
            document.getElementById("user_update_response_message").remove();
        }
    }
}


function change_password() {
    const current_password_change_container = document.getElementById("password_change_container");
    const new_password_field = document.createElement("input");
    new_password_field.id = "new_password_field";
    new_password_field.type = "password";
    new_password_field.placeholder = "input new password";

    const retype_new_password_field = document.createElement("input");
    retype_new_password_field.id = "retype_new_password_field";
    retype_new_password_field.type = "password";
    retype_new_password_field.placeholder = "retype new password";

    current_password_change_container.appendChild(new_password_field);
    current_password_change_container.appendChild(retype_new_password_field);

    const apply_changes_button = document.getElementById("apply_changes_button");
    if (!apply_changes_button) {
        create_apply_and_cancel_buttons();
    }
    document.getElementById("current_password_change_button").disabled = true;
    document.getElementById("current_password_change_cancel_button").disabled = false;
}


function cancel_change_password() {
    const current_password_change_container = document.getElementById("password_change_container");
    current_password_change_container.innerHTML = "";
    document.getElementById("current_password_change_button").disabled = false;
    document.getElementById("current_password_change_cancel_button").disabled = true;

    if (document.getElementById("current_password_change_cancel_button").disabled && 
        document.getElementById("current_email_edit_cancel_button").disabled && 
        document.getElementById("current_user_name_edit_cancel_button").disabled) {

        document.getElementById("apply_changes_button").remove();  
        document.getElementById("cancel_changes_button").remove();

        if (document.getElementById("user_update_response_message")) {
            document.getElementById("user_update_response_message").remove();
        }
    }
}


function create_apply_and_cancel_buttons() {
    const apply_cancel_container = document.getElementById("apply_cancel_container");
    const apply_changes_button = document.createElement("button");
    apply_changes_button.id = "apply_changes_button";
    apply_changes_button.innerHTML = "Apply";

    apply_changes_button.addEventListener("click", function () {

        const current_user_name_editing_field = document.getElementById("current_user_name_editing_field");
        if (current_user_name_editing_field) {
            if (current_user_name_editing_field.value == "")
            {
                alert("Please fill all required fields");
                return false;
            }
        }

        const current_email_editing_field = document.getElementById("current_email_editing_field");
        if (current_email_editing_field) {
            if (!check_email(current_email_editing_field.value)) {
                alert("You have entered an invalid email address");
                return false;
            }
        }

        const new_password_field = document.getElementById("new_password_field");
        const retype_new_password_field = document.getElementById("retype_new_password_field");
        if (new_password_field && retype_new_password_field) {
            if (new_password_field.value == "" || retype_new_password_field.value == "")
            {
                alert("Please fill all required fields");
                return false;
            }
            if (new_password_field.value != retype_new_password_field.value) {
                alert("Password doesn't match");
                return false;
            }
        }

        const edited_user_data = {
            "edited_user_name": current_user_name_editing_field ? current_user_name_editing_field.value : null,
            "edited_email": current_email_editing_field ? current_email_editing_field.value : null,
            "edited_password": new_password_field ? new_password_field.value : null
        };

        const token = localStorage.getItem("authorization");  
        const url = "/auth/update_user";
        const my_settings = {
            method: "POST",
            headers: {"Content-Type": "application/json", "authorization": token},
        body: JSON.stringify(edited_user_data)

        };

        fetch(url, my_settings)
            .then(response => {
                if (response.ok) {
                    return response.text()
                    .then(result => {
                        localStorage.removeItem("authorization");
                        is_logged_in = false;
                        apply_cancel_container.innerHTML = "";
                        document.getElementById("user_info").innerHTML = result;
                    })
                }
                else {
                    return response.text()
                    .then(result => {
                        if (result == "Session has expired, please login again.") {
                            is_logged_in = false;
                            window.location = "../";
                        }
                        else if (document.getElementById("user_update_response_message")) {
                            user_update_response_message.innerHTML = result;
                        }
                        else {
                            const user_update_response_message = document.createElement("p");
                            user_update_response_message.id = "user_update_response_message";
                            apply_cancel_container.appendChild(user_update_response_message);
                            user_update_response_message.innerHTML = result;
                        }
                    })
                }
            });

    });

    const cancel_changes_button = document.createElement("button");
    cancel_changes_button.id = "cancel_changes_button";
    cancel_changes_button.innerHTML = "Cancel";
    cancel_changes_button.addEventListener("click", function () {
        location.reload();
    });
    apply_cancel_container.appendChild(apply_changes_button);
    apply_cancel_container.appendChild(cancel_changes_button);
}
// *** user data editing end


// *** check edited email start
function check_email(email) {
    const re = /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    return re.test(String(email).toLowerCase());
}
// *** check edited email end


// *** show hide all users start
function show_all_users() {

    const token = localStorage.getItem("authorization");  
    const url = "/auth/all_users";
    const my_settings = {
        method: "GET",
        headers: {"authorization": token}
    };

    fetch(url, my_settings)
    .then(response => {
        if (response.ok) {
            return response.text();
        }
        else {
            return response.text();
        }
    })

    .then(result => {
        document.getElementById("all_users").innerHTML = result;
        document.getElementById("show_all_users_info_button").disabled = true;
        document.getElementById("hide_all_users_info_button").disabled = false;
    });
}


function hide_all_users() {
    document.getElementById("all_users").innerHTML = "";
    document.getElementById("show_all_users_info_button").disabled = false;
    document.getElementById("hide_all_users_info_button").disabled = true;
}
// *** show hide all users end


// *** change user status start
function change_user_status(id) {

    const token = localStorage.getItem("authorization");  
    const url = "/auth/change_user_status";
    const required_user_data = { "uid": id };
    const my_settings = {
        method: "POST",
        headers: {"Content-Type": "application/json", "authorization": token},
        body: JSON.stringify(required_user_data)
    };

    fetch(url, my_settings)
    .then(response => {
        if (response.ok) {
            return response.text();
        }
        else {
            return response.text();
        }
    })
    .then(result => {
        alert(result);
        show_all_users();
    })
}
// *** change user status end
