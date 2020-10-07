use yew::prelude::*;

use crate::types::AuthorizedUserResponse;


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub move_checkers_piece: Callback<String>,
    pub user: Option<AuthorizedUserResponse>,
}


struct State
{
    piece_move: Vec<String>
}


pub struct CheckersBoard
{
    link: ComponentLink<Self>,
    props: Props,
    state: State
}


pub enum Msg
{
    SendMessage,
    Click,
    MouseOver,
    ShowPosition(usize, usize)
}


impl Component for CheckersBoard
{
    type Message = Msg;
    type Properties = Props;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self
        {
            props,
            link,
            state: State { piece_move: Vec::new() }
        }
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
            Msg::Click => yew::services::ConsoleService::log("clicked"),
            Msg::MouseOver => yew::services::ConsoleService::log("mouse over"),
            Msg::ShowPosition(letter_num, num) =>
                {
                    let letters = ["A", "B", "C", "D", "E", "F", "G", "H"];
                    if self.state.piece_move.len() == 1
                    {
                        self.state.piece_move.push(format!("{} {}", letters[letter_num - 1], num));
                        yew::services::ConsoleService::log(&format!("{} {}", self.state.piece_move[0], self.state.piece_move[1]));
                        self.state.piece_move = Vec::new();
                    }
                    else
                    {
                        self.state.piece_move.push(format!("{} {}", letters[letter_num - 1], num));
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
        let letters = ["A", "B", "C", "D", "E", "F", "G", "H"];
        let letters_line: Html =
            {
                (0..8).into_iter().map(|i: usize|
                {
                    html!
                    {
                        <div class="cell_alpha">{ letters[i] }</div>
                    }
                }).collect()
            };

        html!
        {
            <div class="checkers_board_container">
                <div class="line">
                    <div class="cell_num">   </div>
                    { letters_line.clone() }
                </div>
                {
                    for (1..=8).into_iter().rev().map(|i: usize|
                    {
                        html!
                        {
                            <div class="line">
                                <div class="cell_num">{ i }</div>
                                {
                                    for (1..=8).into_iter().map(|j: usize|
                                    {
                                        if (i + j) % 2 == 1
                                        {
                                            html!
                                            {
                                                <div
                                                    class="cell white"
                                                    data-alpha={ j }
                                                    data-num={ i }
                                                    onclick=self.link.callback(move |_| Msg::ShowPosition(j, i))>
                                                </div>
                                            }
                                        }
                                        else
                                        {
                                            html!
                                            {
                                                <div
                                                    class="cell black"
                                                    data-alpha={ j }
                                                    data-num={ i }
                                                    onclick=self.link.callback(move |_| Msg::ShowPosition(j, i))>
                                                </div>
                                            }
                                        }
                                    })
                                }
                                <div class="cell_num">{ i }</div>
                            </div>
                        }
                    })
                }

                <div class="line">
                    <div class="cell_num">   </div>
                    { letters_line }
                </div>
            </div>
        }
    }
}