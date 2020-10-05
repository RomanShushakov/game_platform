use yew::prelude::*;

use crate::types::AuthorizedUserResponse;


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub move_checkers_piece: Callback<String>,
    pub user: Option<AuthorizedUserResponse>,
}


pub struct CheckersBoard
{
    link: ComponentLink<Self>,
    props: Props,
}


pub enum Msg
{
    SendMessage
}


impl Component for CheckersBoard
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::SendMessage =>
                {
                    // self.props.move_piece.emit("E2-E4".to_string());
                    if let Some(user) = &self.props.user
                    {
                        yew::services::ConsoleService::log(&user.user_name);
                    }
                    else
                    {
                        yew::services::ConsoleService::log("unknown user");
                    }

                }
        }
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            true
        }
        else
        {
            false
        }
    }


    fn view(&self) -> Html
    {
        html!
        {
            <div class="checkers_board_container">
                <div class="line" onclick=self.link.callback(|_| Msg::SendMessage)>
                    <div class="cell_num">   </div>
                    <div class="cell_alpha">{ "A" }</div>
                    <div class="cell_alpha">{ "B" }</div>
                    <div class="cell_alpha">{ "C" }</div>
                    <div class="cell_alpha">{ "D" }</div>
                    <div class="cell_alpha">{ "E" }</div>
                    <div class="cell_alpha">{ "F" }</div>
                    <div class="cell_alpha">{ "G" }</div>
                    <div class="cell_alpha">{ "H" }</div>
                </div>
                <div class="line">
                    <div class="cell_num">{ 8 }</div>
                    <div class="cell white" data-alpha="1" data-num="8"></div>
                    <div class="cell black" data-alpha="2" data-num="8"></div>
                    <div class="cell white" data-alpha="3" data-num="8"></div>
                    <div class="cell black" data-alpha="4" data-num="8"></div>
                    <div class="cell white" data-alpha="5" data-num="8"></div>
                    <div class="cell black" data-alpha="6" data-num="8"></div>
                    <div class="cell white" data-alpha="7" data-num="8"></div>
                    <div class="cell black" data-alpha="8" data-num="8"></div>
                    <div class="cell_num">{ 8 }</div>
                </div>
                <div class="line">
                    <div class="cell_num">{ 7 }</div>
                    <div class="cell black" data-alpha="1" data-num="7"></div>
                    <div class="cell white" data-alpha="2" data-num="7"></div>
                    <div class="cell black" data-alpha="3" data-num="7"></div>
                    <div class="cell white" data-alpha="4" data-num="7"></div>
                    <div class="cell black" data-alpha="5" data-num="7"></div>
                    <div class="cell white" data-alpha="6" data-num="7"></div>
                    <div class="cell black" data-alpha="7" data-num="7"></div>
                    <div class="cell white" data-alpha="8" data-num="7"></div>
                    <div class="cell_num">{ 7 }</div>
                </div>
                <div class="line">
                    <div class="cell_num">{ 6 }</div>
                    <div class="cell white" data-alpha="1" data-num="6"></div>
                    <div class="cell black" data-alpha="2" data-num="6"></div>
                    <div class="cell white" data-alpha="3" data-num="6"></div>
                    <div class="cell black" data-alpha="4" data-num="6"></div>
                    <div class="cell white" data-alpha="5" data-num="6"></div>
                    <div class="cell black" data-alpha="6" data-num="6"></div>
                    <div class="cell white" data-alpha="7" data-num="6"></div>
                    <div class="cell black" data-alpha="8" data-num="6"></div>
                    <div class="cell_num">{ 6 }</div>
                </div>
                <div class="line">
                    <div class="cell_num">{ 5 }</div>
                    <div class="cell black" data-alpha="1" data-num="5"></div>
                    <div class="cell white" data-alpha="2" data-num="5"></div>
                    <div class="cell black" data-alpha="3" data-num="5"></div>
                    <div class="cell white" data-alpha="4" data-num="5"></div>
                    <div class="cell black" data-alpha="5" data-num="5"></div>
                    <div class="cell white" data-alpha="6" data-num="5"></div>
                    <div class="cell black" data-alpha="7" data-num="5"></div>
                    <div class="cell white" data-alpha="8" data-num="5"></div>
                    <div class="cell_num">{ 5 }</div>
                </div>
                <div class="line">
                    <div class="cell_num">{ 4 }</div>
                    <div class="cell white" data-alpha="1" data-num="4"></div>
                    <div class="cell black" data-alpha="2" data-num="4"></div>
                    <div class="cell white" data-alpha="3" data-num="4"></div>
                    <div class="cell black" data-alpha="4" data-num="4"></div>
                    <div class="cell white" data-alpha="5" data-num="4"></div>
                    <div class="cell black" data-alpha="6" data-num="4"></div>
                    <div class="cell white" data-alpha="7" data-num="4"></div>
                    <div class="cell black" data-alpha="8" data-num="4"></div>
                    <div class="cell_num">{ 4 }</div>
                </div>
                <div class="line">
                    <div class="cell_num">{ 3 }</div>
                    <div class="cell black" data-alpha="1" data-num="3"></div>
                    <div class="cell white" data-alpha="2" data-num="3"></div>
                    <div class="cell black" data-alpha="3" data-num="3"></div>
                    <div class="cell white" data-alpha="4" data-num="3"></div>
                    <div class="cell black" data-alpha="5" data-num="3"></div>
                    <div class="cell white" data-alpha="6" data-num="3"></div>
                    <div class="cell black" data-alpha="7" data-num="3"></div>
                    <div class="cell white" data-alpha="8" data-num="3"></div>
                    <div class="cell_num">{ 3 }</div>
                </div>
                <div class="line">
                    <div class="cell_num">{ 2 }</div>
                    <div class="cell white" data-alpha="1" data-num="2"></div>
                    <div class="cell black" data-alpha="2" data-num="2"></div>
                    <div class="cell white" data-alpha="3" data-num="2"></div>
                    <div class="cell black" data-alpha="4" data-num="2"></div>
                    <div class="cell white" data-alpha="5" data-num="2"></div>
                    <div class="cell black" data-alpha="6" data-num="2"></div>
                    <div class="cell white" data-alpha="7" data-num="2"></div>
                    <div class="cell black" data-alpha="8" data-num="2"></div>
                    <div class="cell_num">{ 2 }</div>
                </div>
                <div class="line">
                    <div class="cell_num">{ 1 }</div>
                    <div class="cell black" data-alpha="1" data-num="1"></div>
                    <div class="cell white" data-alpha="2" data-num="1"></div>
                    <div class="cell black" data-alpha="3" data-num="1"></div>
                    <div class="cell white" data-alpha="4" data-num="1"></div>
                    <div class="cell black" data-alpha="5" data-num="1"></div>
                    <div class="cell white" data-alpha="6" data-num="1"></div>
                    <div class="cell black" data-alpha="7" data-num="1"></div>
                    <div class="cell white" data-alpha="8" data-num="1"></div>
                    <div class="cell_num">{ 1 }</div>
                </div>
                <div class="line">
                    <div class="cell_num">   </div>
                    <div class="cell_alpha">{ "A" }</div>
                    <div class="cell_alpha">{ "B" }</div>
                    <div class="cell_alpha">{ "C" }</div>
                    <div class="cell_alpha">{ "D" }</div>
                    <div class="cell_alpha">{ "E" }</div>
                    <div class="cell_alpha">{ "F" }</div>
                    <div class="cell_alpha">{ "G" }</div>
                    <div class="cell_alpha">{ "H" }</div>
                </div>
            </div>
        }
    }
}