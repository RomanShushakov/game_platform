use yew::prelude::*;


pub struct AllUsers;


impl Component for AllUsers
{
    type Message = ();
    type Properties = ();


    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self
    {
        Self {  }
    }


    fn update(&mut self, _msg: Self::Message) -> ShouldRender
    {
        true
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }


    fn view(&self) -> Html
    {
        html!
        {
            <div class="all_users_info">
              <button>{ "show all users" }</button>
              <button disabled=true>{ "hide all users" }</button>
            </div>
        }
    }
}
