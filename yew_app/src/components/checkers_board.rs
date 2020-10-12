use yew::prelude::*;
use serde_json;


use crate::types::{AuthorizedUserResponse, WsRequest, WsResponse, PieceColor, CheckerPosition, GameData};
use crate::pages::{GameAction, GAME_NAME, ChatAction};


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub user: Option<AuthorizedUserResponse>,
    pub is_in_game: bool,
    pub piece_color: Option<PieceColor>,
    pub send_websocket_data: Callback<WsRequest>,
    pub reset_websocket_game_response: Callback<()>,
    pub websocket_game_response: Option<WsResponse>,
    pub leave_game: Callback<()>,
}


enum GameResult
{
    Win,
    Lose,
}


struct State
{
    piece_move: Vec<CheckerPosition>,
    white_checkers_positions: Vec<CheckerPosition>,
    black_checkers_positions: Vec<CheckerPosition>,
    is_steps_order_defined: bool,
    is_my_step: bool,
    game_result: Option<GameResult>,
}


impl State
{
    fn init() -> Self
    {
        State
        {
            piece_move: Vec::new(),
            white_checkers_positions: vec![
                    CheckerPosition { column: 1, line: 1 },
                    CheckerPosition { column: 1, line: 3 },
                    CheckerPosition { column: 2, line: 2 },
                    CheckerPosition { column: 3, line: 1 },
                    CheckerPosition { column: 3, line: 3 },
                    CheckerPosition { column: 4, line: 2 },
                    CheckerPosition { column: 5, line: 1 },
                    CheckerPosition { column: 5, line: 3 },
                    CheckerPosition { column: 6, line: 2 },
                    CheckerPosition { column: 7, line: 1 },
                    CheckerPosition { column: 7, line: 3 },
                    CheckerPosition { column: 8, line: 2 },
                ],
            black_checkers_positions: vec![
                    CheckerPosition { column: 1, line: 7 },
                    CheckerPosition { column: 2, line: 6 },
                    CheckerPosition { column: 2, line: 8 },
                    CheckerPosition { column: 3, line: 7 },
                    CheckerPosition { column: 4, line: 6 },
                    CheckerPosition { column: 4, line: 8 },
                    CheckerPosition { column: 5, line: 7 },
                    CheckerPosition { column: 6, line: 6 },
                    CheckerPosition { column: 6, line: 8 },
                    CheckerPosition { column: 7, line: 7 },
                    CheckerPosition { column: 8, line: 6 },
                    CheckerPosition { column: 8, line: 8 },
                ],
            is_steps_order_defined: false,
            is_my_step: false,
            game_result: None,
        }
    }
}


pub struct CheckersBoard
{
    link: ComponentLink<Self>,
    props: Props,
    state: State
}


pub enum Msg
{
    MoveCheckerPiece(usize, usize),
    LeaveGame,
}


impl CheckersBoard
{
    fn view_black_cells(&self, column: usize, line: usize) -> Html
    {
        yew::services::ConsoleService::log("black cells called");
        let white_checker: Html =
            {
                if let Some(color) = &self.props.piece_color
                {
                    match color
                    {
                        PieceColor::White => html! { <div class="checker_my_white"></div> },
                        PieceColor::Black => html! { <div class="checker_white"></div> },
                    }
                }
                else { html! { <div class="checker_white"></div> } }
            };
        let black_checker: Html =
            {
                if let Some(color) = &self.props.piece_color
                {
                    match color
                    {
                        PieceColor::White => html! { <div class="checker_black"></div> },
                        PieceColor::Black => html! { <div class="checker_my_black"></div> },
                    }
                }
                else { html! { <div class="checker_black"></div> } }
            };

        html!
        {
            if let Some(_) = self.state.white_checkers_positions
                .iter()
                .position(|checker_position| *checker_position == CheckerPosition { column, line })
            {
                html!
                {
                    <div
                        class="cell black"
                        onclick=self.link.callback(move |_| Msg::MoveCheckerPiece(column, line))>
                        { white_checker.clone() }
                    </div>
                }
            }
            else if let Some(_) = self.state.black_checkers_positions
                .iter()
                .position(|checker_position| *checker_position == CheckerPosition { column, line })
            {
                html!
                {
                    <div
                        class="cell black"
                        onclick=self.link.callback(move |_| Msg::MoveCheckerPiece(column, line))>
                        { black_checker.clone() }
                    </div>
                }
            }
            else
            {
                html!
                {
                    <div
                        class="cell black"
                        onclick=self.link.callback(move |_| Msg::MoveCheckerPiece(column, line))>
                    </div>
                }
            }
        }
    }


    fn checkers_board_header_view(&self) -> Html
    {
        if let Some(result) = &self.state.game_result
        {
            match result
            {
                GameResult::Win =>
                    {
                        html!
                        {
                            <div class="container">
                                <p>{ "You win !!!" }</p>
                                <button
                                    disabled=!self.props.is_in_game
                                    onclick=self.link.callback(|_| Msg::LeaveGame)>
                                    { "Quit Game" }
                                </button>
                            </div>
                        }
                    },
                GameResult::Lose =>
                    {
                        html!
                        {
                            <div class="container">
                                <p>{ "You lose :-(" }</p>
                                <button disabled=!self.props.is_in_game>
                                    { "Quit Game" }
                                </button>
                            </div>
                        }
                    },
            }
        }
        else
        {
            html!
            {
                <div class="container">
                    <button
                        disabled=!self.props.is_in_game
                        onclick=self.link.callback(|_| Msg::LeaveGame)>
                        { "Surrender" }
                    </button>
                </div>
            }
        }
    }
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
            state: State::init(),
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        if let Some(color) = &self.props.piece_color
            {
                if !self.state.is_steps_order_defined
                {
                    match color
                    {
                        PieceColor::White => self.state.is_my_step = true,
                        _ => (),
                    }
                    self.state.is_steps_order_defined = true;
                }
            }

        match msg
        {
            Msg::MoveCheckerPiece(column, line) =>
                {
                    if self.state.is_my_step
                    {
                        if self.state.piece_move.len() == 1
                        {
                            self.state.piece_move.push(CheckerPosition { column, line });
                            match self.props.piece_color
                            {
                                Some(PieceColor::White) =>
                                    {
                                        if let Some(idx) = self.state.white_checkers_positions
                                            .iter()
                                            .position(
                                                |position|
                                                    position.column == self.state.piece_move[0].column
                                                        &&
                                                    position.line == self.state.piece_move[0].line)
                                        {
                                            self.state.white_checkers_positions[idx].column = self.state.piece_move[1].column;
                                            self.state.white_checkers_positions[idx].line = self.state.piece_move[1].line;
                                        }
                                    },
                                Some(PieceColor::Black) =>
                                    {
                                        if let Some(idx) = self.state.black_checkers_positions
                                            .iter()
                                            .position(
                                                |position|
                                                    position.column == self.state.piece_move[0].column
                                                        &&
                                                    position.line == self.state.piece_move[0].line)
                                        {
                                            self.state.black_checkers_positions[idx].column = self.state.piece_move[1].column;
                                            self.state.black_checkers_positions[idx].line = self.state.piece_move[1].line;
                                        }
                                    },
                                None => (),
                            }
                            let data =
                                {
                                    serde_json::to_string(
                                    &GameData
                                        {
                                            opponent_piece_color: self.props.piece_color.clone().unwrap(),
                                            piece_previous_position: self.state.piece_move[0].to_owned(),
                                            piece_new_position: self.state.piece_move[1].to_owned(),
                                            captured_piece_position: None,
                                            is_opponent_step: false,
                                        }).unwrap()
                                };
                            let request = WsRequest
                                {
                                    action: GameAction::SendCheckerPieceMove.as_str(),
                                    data
                                };
                            self.props.send_websocket_data.emit(request);
                            self.state.piece_move = Vec::new();
                            self.state.is_my_step = false;
                        }
                        else
                        {
                            self.state.piece_move.push(CheckerPosition { column, line });
                        }
                    }
                    else { return false; }
                },
            Msg::LeaveGame =>
                {
                    self.state = State::init();
                    self.props.leave_game.emit(());
                    let request = WsRequest { action: GameAction::SendLeaveGameMessage.as_str(), data: GAME_NAME.to_string() };
                    self.props.send_websocket_data.emit(request);
                    if let Some(user) = &self.props.user
                    {
                        let request_online_users = WsRequest { action: ChatAction::RequestOnlineUsers.as_str(), data: format!("{}", user.user_name) };
                        self.props.send_websocket_data.emit(request_online_users);
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
            if let Some(response) = self.props.websocket_game_response.clone()
            {
                if response.action == GameAction::ReceivedCheckerPieceMove.as_str()
                {
                    self.props.reset_websocket_game_response.emit(());
                    self.state.is_my_step = true;
                    let game_data: GameData = serde_json::from_str(&response.data).unwrap();

                    match game_data.opponent_piece_color
                    {
                        PieceColor::White =>
                            {
                                if let Some(idx) = self.state.white_checkers_positions
                                    .iter()
                                    .position(
                                        |position|
                                            position.column == game_data.piece_previous_position.column
                                                &&
                                            position.line == game_data.piece_previous_position.line)
                                {
                                    self.state.white_checkers_positions[idx].column = game_data.piece_new_position.column;
                                    self.state.white_checkers_positions[idx].line = game_data.piece_new_position.line;
                                }
                            },
                        PieceColor::Black =>
                            {
                                if let Some(idx) = self.state.black_checkers_positions
                                    .iter()
                                    .position(
                                        |position|
                                            position.column == game_data.piece_previous_position.column
                                                &&
                                            position.line == game_data.piece_previous_position.line)
                                {
                                    self.state.black_checkers_positions[idx].column = game_data.piece_new_position.column;
                                    self.state.black_checkers_positions[idx].line = game_data.piece_new_position.line;
                                }
                            },
                    }

                }
                else if response.action == GameAction::ReceivedLeaveGameMessage.as_str()
                {
                    self.state.game_result = Some(GameResult::Win);
                }
                else { return false; }
            }
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
        let (number_sequence, letter_sequence): (Vec<usize>, Vec<usize>) =
            {
                if let Some(color) = &self.props.piece_color
                {
                    match color
                    {
                        PieceColor::White => ((1..=8).into_iter().rev().collect(), (1..=8).into_iter().collect()),
                        PieceColor::Black => ((1..=8).into_iter().collect(), (1..=8).into_iter().rev().collect())
                    }
                }
                else { ((1..=8).into_iter().rev().collect(), (1..=8).into_iter().collect()) }
            };


        let letters_line: Html =
            {
                letter_sequence.clone().into_iter().map(|i: usize|
                {
                    html!
                    {
                        <div class="cell_alpha">{ letters[i - 1] }</div>
                    }
                }).collect()
            };


        html!
        {
            <div class="checkers_board_container">

                { self.checkers_board_header_view() }

                <div class="line">
                    <div class="cell_num">   </div>
                    { letters_line.clone() }
                </div>
                {
                    for number_sequence.into_iter().map(|i: usize|
                    {
                        html!
                        {
                            <div class="line">
                                <div class="cell_num">{ i }</div>
                                {
                                    for letter_sequence.clone().into_iter().map(|j: usize|
                                    {
                                        if (i + j) % 2 == 1
                                        {
                                            html! { <div class="cell white">  </div> }
                                        }
                                        else
                                        {
                                            if self.props.is_in_game
                                            {
                                                { self.view_black_cells(j, i) }
                                            }
                                            else
                                            {
                                                html! { <div class="cell black">  </div> }
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
