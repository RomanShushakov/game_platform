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
        // add_startup_greeting(user_data["user_name"]); 
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
                    enable_apply_reset_changes_buttons();
                });
        }
        else {
            window.location = "../";
        }
    });
};
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
    activate_apply_changes_button();
    document.getElementById("user_data_change_response_message").innerHTML = "";

    document.getElementById("current_user_name_editing_field").disabled = false;
    document.getElementById("current_user_name_edit_button").disabled = true;
    document.getElementById("current_user_name_edit_cancel_button").disabled = false;
}


function cancel_edit_name() {
    document.getElementById("current_user_name_editing_field").disabled = true;
    document.getElementById("current_user_name_editing_field").value = "";
    document.getElementById("current_user_name_edit_button").disabled = false;
    document.getElementById("current_user_name_edit_cancel_button").disabled = true;

    if (document.getElementById("current_password_change_cancel_button").disabled && 
        document.getElementById("current_email_edit_cancel_button").disabled && 
        document.getElementById("current_user_name_edit_cancel_button").disabled) {

        document.getElementById("apply_changes_button").disabled = true; 
        document.getElementById("user_data_change_response_message").innerHTML = "";
    }
}


function edit_email() {
    activate_apply_changes_button();
    document.getElementById("user_data_change_response_message").innerHTML = "";

    document.getElementById("current_email_editing_field").disabled = false;
    document.getElementById("current_email_edit_button").disabled = true;
    document.getElementById("current_email_edit_cancel_button").disabled = false;
}


function cancel_edit_email() {
    document.getElementById("current_email_editing_field").disabled = true;
    document.getElementById("current_email_editing_field").value = "";
    document.getElementById("current_email_edit_button").disabled = false;
    document.getElementById("current_email_edit_cancel_button").disabled = true;

    if (document.getElementById("current_password_change_cancel_button").disabled && 
        document.getElementById("current_email_edit_cancel_button").disabled && 
        document.getElementById("current_user_name_edit_cancel_button").disabled) {

        document.getElementById("apply_changes_button").disabled = true; 
        document.getElementById("user_data_change_response_message").innerHTML = "";
    }
}


function change_password() {
    activate_apply_changes_button();
    document.getElementById("user_data_change_response_message").innerHTML = "";

    document.getElementById("new_password_field").disabled = false;
    document.getElementById("retype_new_password_field").disabled = false;
    document.getElementById("current_password_change_button").disabled = true;
    document.getElementById("current_password_change_cancel_button").disabled = false;
}


function cancel_change_password() {
    document.getElementById("new_password_field").disabled = true;
    document.getElementById("new_password_field").value = "";
    document.getElementById("retype_new_password_field").disabled = true;
    document.getElementById("retype_new_password_field").value = "";
    document.getElementById("current_password_change_button").disabled = false;
    document.getElementById("current_password_change_cancel_button").disabled = true;

    if (document.getElementById("current_password_change_cancel_button").disabled && 
        document.getElementById("current_email_edit_cancel_button").disabled && 
        document.getElementById("current_user_name_edit_cancel_button").disabled) {

        document.getElementById("apply_changes_button").disabled = true;  
        document.getElementById("user_data_change_response_message").innerHTML = "";
    }
}


function activate_apply_changes_button() {
    const apply_changes_button = document.getElementById("apply_changes_button");
    apply_changes_button.disabled = false;
}


function enable_apply_reset_changes_buttons() {
    const apply_changes_button = document.getElementById("apply_changes_button");
    apply_changes_button.addEventListener("click", function () {
        const current_user_name_editing_field = document.getElementById("current_user_name_editing_field");
        if (!current_user_name_editing_field.disabled) {
            if (current_user_name_editing_field.value == "")
            {
                alert("Please fill all required fields.");
                return false;
            }
        }
    
        const current_email_editing_field = document.getElementById("current_email_editing_field");
        if (!current_email_editing_field.disabled) {
            if (!check_email(current_email_editing_field.value)) {
                alert("You have entered an invalid email address.");
                return false;
            }
        }
    
        const new_password_field = document.getElementById("new_password_field");
        const retype_new_password_field = document.getElementById("retype_new_password_field");
        if (!new_password_field.disabled && !retype_new_password_field.disabled) {
            if (new_password_field.value == "" || retype_new_password_field.value == "")
            {
                alert("Please fill all required fields.");
                return false;
            }
            if (new_password_field.value != retype_new_password_field.value) {
                alert("Password doesn't match.");
                return false;
            }
        }
    
        const edited_user_data = {
            "edited_user_name": !current_user_name_editing_field.disabled ? current_user_name_editing_field.value : null,
            "edited_email": !current_email_editing_field.disabled ? current_email_editing_field.value : null,
            "edited_password": !new_password_field.disabled ? new_password_field.value : null
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
                        document.getElementById("user_name_editing_container").remove();
                        document.getElementById("email_editing_container").remove();
                        document.getElementById("password_editing_container").remove();
                        document.getElementById("apply_cancel_container").remove();
                        document.getElementById("user_data_change_response_message").innerHTML = result;
                    })
                }
                else {
                    return response.text()
                    .then(result => {
                        if (result == "Session has expired, please login again.") {
                            is_logged_in = false;
                            window.location = "../";
                        }
                        else {
                            document.getElementById("user_data_change_response_message").innerHTML = result;
                        }
                    })
                }
            });
    
    });

    const reset_changes_button = document.getElementById("reset_changes_button");
    reset_changes_button.addEventListener("click", function () {
        location.reload();
    });
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
