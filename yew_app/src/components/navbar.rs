use yew::prelude::*;

use crate::route::Route;
use yew_router::components::RouterAnchor;
use crate::types::AuthorizedUserResponse;


#[derive(Properties, Clone)]
pub struct Props
{
    pub user: Option<AuthorizedUserResponse>,
    pub token: Option<String>
}


pub struct NavBar
{
    link: ComponentLink<Self>,
    props: Props
}


impl NavBar
{
    fn view_logo(&self) -> Html
    {
        html!
        {
            <svg width="39" height="35" viewBox="0 0 39 35" fill="rgb(187, 167, 167)">
              <path d="M11.9123 16.9191C14.2981 15.296 16.2735 13.1649 18.5667 11.4324C19.9554 10.3833 22.2212 9.42767 24 9.45951C26.8757 9.511 29.7818 10.875 31.7586 12.9013C33.5302 14.7172 35.3006 16.3033 36.7824 18.3592C38.9384 21.3507 37.9564 25.3446 35.9707 28.1662C34.7256 29.9353 32.6065 31.2593 30.7129 32.2848C28.3356 33.5723 25.9333 34.1429 23.2175 33.9697C21.1403 33.8372 19.3189 33.607 17.3528 32.9617C15.7363 32.4311 14.2411 31.7797 12.7094 31.0535C11.6148 30.5346 10.7932 29.9444 9.93789 29.1022C9.20189 28.3775 9.10677 27.8288 8.81906 26.8845C8.47325 25.7495 8.44647 24.6817 8.87756 23.5939C9.37976 22.3268 9.56235 20.9295 10.3328 19.7705C10.8447 19.0003 11.3729 17.8672 12.0439 17.2432C12.3046 17.0007 12.5566 16.7627 12.8264 16.5303C12.9142 16.4546 13.0969 16.1338 13.0969 16.2711" stroke="black" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M19.6782 24.0475C19.6782 24.9592 19.5629 25.7655 20.1316 26.5389C20.4212 26.9327 21.1998 27.5469 21.7258 27.5469C22.3626 27.5469 22.949 27.7639 23.5978 27.6477C24.1025 27.5574 24.9696 27.5902 25.404 27.3525C26.507 26.7492 28.1991 27.0729 28.9506 25.862C29.5493 24.8974 30.193 23.3819 29.9305 22.2186C29.6161 20.8254 28.567 20.0649 27.2541 19.7417C26.5171 19.5603 26.076 19.3142 25.2797 19.3889C24.7579 19.4378 23.8894 19.7 23.4954 20.0297C23.2818 20.2085 22.947 20.2263 22.6983 20.3609C22.2533 20.602 21.9404 20.9748 21.5283 21.2538C20.9868 21.6204 20.3343 21.922 19.9415 22.4634C19.7329 22.751 19.0201 23.0939 19.0201 23.4643C19.0201 23.8604 19.1342 24.5602 19.5466 24.6956" stroke="black" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M23.3638 23.9179C22.9752 23.9908 22.4943 23.9496 22.1206 24.0547C21.9467 24.1037 22.4284 24.8428 22.5082 24.9476C22.9741 25.5592 23.1975 24.8782 23.627 24.6668C24.0781 24.4447 23.6232 23.9779 23.4223 23.7235C23.2416 23.4948 22.5526 23.7008 22.4424 23.9179" stroke="black" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M26.3912 24.0475C26.7605 24.3317 26.8779 24.4719 27.2833 24.5948C27.6541 24.7071 27.5595 24.5459 27.7732 24.242C28.2087 23.6226 27.8025 23.1658 27.0785 23.1403C26.2499 23.1112 25.5376 24.566 26.3912 24.566" stroke="black" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M16.7825 19.2244C15.3526 20.4211 16.7462 22.7595 18.2303 21.2981C18.5423 20.991 18.8091 20.1383 18.4643 19.7717C18.2869 19.583 18.0863 19.354 17.8281 19.354C17.5766 19.354 17.1622 19.4538 17.0457 19.2244" stroke="black" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M28.425 17.8381C28.962 18.4382 29.3104 18.5293 30.1003 18.4789C30.7454 18.4378 30.7675 17.1421 30.5931 16.7168C30.5015 16.4934 29.9226 16.1295 29.7062 16.1295C29.3884 16.1295 29.0498 16.5249 28.8138 16.7168C28.3877 17.0631 27.6776 17.6038 28.6221 17.9449" stroke="black" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M14.0183 15.3638C13.7761 14.7402 13.5695 13.9977 13.1408 13.4629C12.7311 12.9519 12.248 12.5025 11.8465 11.9868C11.3944 11.4063 11.2545 10.7921 10.8593 10.2083C10.1125 9.10543 8.79342 8.31349 7.57592 7.78901C6.31449 7.2456 4.8615 7.15119 3.51744 6.91056C2.78091 6.7787 2.07237 6.49801 1.38217 6.23372C0.450799 5.87708 1.49681 7.46239 1.63811 7.645C2.24334 8.42718 2.95636 9.13335 3.75876 9.72592C4.6015 10.3483 5.25712 11.5716 5.85747 12.4117C6.71443 13.6108 7.15059 15.002 7.95618 16.1919C8.29733 16.6958 8.93686 17.0113 9.2505 17.5672C9.46246 17.9428 10.0424 18.4655 10.4644 18.604" stroke="black" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M33.1041 14.0678C35.122 13.7101 35.4083 12.1968 35.9706 10.5972C36.3593 9.49142 37.1507 8.48506 37.477 7.32818C37.7838 6.24058 38.2226 5.08501 37.4185 4.088C36.759 3.27021 36.2065 2.32803 35.4149 1.69026C35.2614 1.56662 34.2105 0.930618 34.1644 1.00622C33.8861 1.46296 34.0255 2.2283 34.0255 2.73432C34.0255 3.75078 33.8533 4.87098 33.499 5.8305C33.0165 7.13699 32.1926 8.10303 31.254 9.14269C30.5451 9.92801 29.808 11.0022 28.7604 11.346" stroke="black" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M19.0057 29.4911C21.3573 31.3154 24.3782 31.7884 27.1811 30.7151C28.3651 30.2618 29.5769 29.969 30.5157 29.0446C31.0036 28.5642 31.4429 28.108 32.0221 27.7414C32.7182 27.3007 31.491 27.7742 31.3274 27.799C30.4299 27.9349 29.6565 28.4911 28.7533 28.7134C28.1463 28.8628 27.6392 29.188 27.0641 29.3902C26.5776 29.5613 26.0595 29.8514 25.587 29.9807C23.538 30.5411 21.4836 29.8799 19.4006 29.8799" stroke="black" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        }
    }

    fn view_auth_buttons(&self) -> Html
    {
        type Anchor = RouterAnchor<Route>;

        html!
        {

            if let Some(user) = &self.props.user
            {
                html!
                {
                  <>
                    <button class="button">{ "Sign out" }</button>
                    <Anchor route=Route::UserInfo>
                      <button class="button">{ user.user_name.to_string() }</button>
                    </Anchor>
                  </>
                }

            }
            else
            {
                html!
                {
                  <>
                    <Anchor route=Route::SignInUser>
                      <button class="button">{ "Sign in" }</button>
                    </Anchor>
                    <Anchor route=Route::RegisterUser>
                      <button class="button">{ "Register" }</button>
                    </Anchor>
                  </>
                }

            }
            // <button id="user_info_button" class="button">{ "Guest" }</button>
        }
    }
}


impl Component for NavBar
{
    type Message = ();
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link }
    }


    fn update(&mut self, _msg: Self::Message) -> ShouldRender
    {
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        self.props = props;
        true
    }


    fn view(&self) -> Html
    {
        type Anchor = RouterAnchor<Route>;

        html!
        {
            <header class="header">

                <div class="container">

                  <div class="logo menu_item">
                    { self.view_logo() }
                  </div>
                  <nav class="header_navigation">
                    <ul class="header_list">
                      <li class="header_list_item">
                        <Anchor route=Route::HomePage>{ "Home" }</Anchor>
                      </li>
                      <li class="header_list_item">
                        <a href="#">{ "Checkers" }</a>
                      </li>
                    </ul>
                  </nav>

                  <div class="authentication_buttons">
                    { self.view_auth_buttons() }
                  </div>
                </div>
            </header>

        }
    }
}
