use yew::prelude::*;


pub struct RegisterUser;


impl Component for RegisterUser
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
            <h2>{ "RegisterUser" }</h2>
        }
    }
}
