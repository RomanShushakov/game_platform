use std::collections::HashMap;
use yew::prelude::*;
use serde_json;


use crate::types::{
    AuthorizedUserResponse, WsRequest, WsResponse, PieceColor, CheckerPiece, GameData, CheckerPosition,
    AllowableMove
};
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
    checker_pieces: HashMap<PieceColor, Vec<CheckerPiece>>,
    is_steps_order_defined: bool,
    is_my_step: bool,
    allowable_moves: Option<Vec<AllowableMove>>,
    game_result: Option<GameResult>,
}


impl State
{
    fn init() -> Self
    {
        let mut checker_pieces = HashMap::new();
        checker_pieces.insert(
            PieceColor::White,
            vec![
                CheckerPiece { id: 1, is_crowned: false, position: CheckerPosition { column: 1, line: 1 } },
                CheckerPiece { id: 2, is_crowned: false, position: CheckerPosition { column: 1, line: 3 } },
                CheckerPiece { id: 3, is_crowned: false, position: CheckerPosition { column: 2, line: 2 } },
                CheckerPiece { id: 4, is_crowned: false, position: CheckerPosition { column: 3, line: 1 } },
                CheckerPiece { id: 5, is_crowned: false, position: CheckerPosition{ column: 3, line: 3 } },
                CheckerPiece { id: 6, is_crowned: false, position: CheckerPosition { column: 4, line: 2 } },
                CheckerPiece { id: 7, is_crowned: false, position: CheckerPosition { column: 5, line: 1 } },
                CheckerPiece { id: 8, is_crowned: false, position: CheckerPosition { column: 5, line: 3 } },
                CheckerPiece { id: 9, is_crowned: false, position: CheckerPosition { column: 6, line: 2 } },
                CheckerPiece { id: 10, is_crowned: false, position: CheckerPosition { column: 7, line: 1 } },
                CheckerPiece { id: 11, is_crowned: false, position: CheckerPosition { column: 7, line: 3 } },
                CheckerPiece { id: 12, is_crowned: false, position: CheckerPosition { column: 8, line: 2 } },
            ],);
        checker_pieces.insert(
            PieceColor::Black,
            vec![
                CheckerPiece { id: 1, is_crowned: false, position: CheckerPosition { column: 1, line: 7 } },
                CheckerPiece { id: 2, is_crowned: false, position: CheckerPosition { column: 2, line: 6 } },
                CheckerPiece { id: 3, is_crowned: false, position: CheckerPosition { column: 2, line: 8 } },
                CheckerPiece { id: 4, is_crowned: false, position: CheckerPosition { column: 3, line: 7 } },
                CheckerPiece { id: 5, is_crowned: false, position: CheckerPosition { column: 4, line: 6 } },
                CheckerPiece { id: 6, is_crowned: false, position: CheckerPosition { column: 4, line: 8 } },
                CheckerPiece { id: 7, is_crowned: false, position: CheckerPosition { column: 5, line: 7 } },
                CheckerPiece { id: 8, is_crowned: false, position: CheckerPosition { column: 6, line: 6 } },
                CheckerPiece { id: 9, is_crowned: false, position: CheckerPosition { column: 6, line: 8 } },
                CheckerPiece { id: 10, is_crowned: false, position: CheckerPosition { column: 7, line: 7 } },
                CheckerPiece { id: 11, is_crowned: false, position: CheckerPosition { column: 8, line: 6 } },
                CheckerPiece { id: 12, is_crowned: false, position: CheckerPosition { column: 8, line: 8 } },
                ],);

        State
        {
            piece_move: Vec::new(),
            checker_pieces,
            is_steps_order_defined: false,
            is_my_step: false,
            allowable_moves: None,
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


fn find_position(checker_pieces: &Vec<CheckerPiece>, position: &CheckerPosition) -> Option<usize>
{
    checker_pieces.iter().position(|checker_piece| &checker_piece.position == position)
}


fn is_allowable_position(position: &CheckerPosition) -> bool
{
    (1 <= position.column) && (position.column <= 8) && (1 <= position.line) && (position.line <= 8)
}



impl CheckersBoard
{
    fn view_black_cells(&self, column: usize, line: usize) -> Html
    {
        // yew::services::ConsoleService::log("black cells called");
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
        let white_checker_active: Html = html! { <div class="checker_my_white_active"></div> };
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
        let black_checker_active: Html = html! { <div class="checker_my_black_active"></div> };

        html!
        {
            if let Some(_) = self.state.checker_pieces[&PieceColor::White]
                .iter()
                .position(|checker_piece| checker_piece.position == CheckerPosition { column, line })
            {
                html!
                {
                    <div
                        class="cell black"
                        onclick=self.link.callback(move |_| Msg::MoveCheckerPiece(column, line))>
                        {
                            if !self.state.piece_move.is_empty() && (self.state.piece_move[0] == CheckerPosition { column, line })
                            {
                                white_checker_active.clone()
                            }
                            else
                            {
                                white_checker.clone()
                            }
                        }
                    </div>
                }
            }
            else if let Some(_) = self.state.checker_pieces[&PieceColor::Black]
                .iter()
                .position(|checker_piece| checker_piece.position == CheckerPosition { column, line })
            {
                html!
                {
                    <div
                        class="cell black"
                        onclick=self.link.callback(move |_| Msg::MoveCheckerPiece(column, line))>
                        {
                            if !self.state.piece_move.is_empty() && (self.state.piece_move[0] == CheckerPosition { column, line })
                            {
                                black_checker_active.clone()
                            }
                            else
                            {
                                black_checker.clone()
                            }
                        }
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


    fn can_capturing_checkers(&self) -> Option<Vec<AllowableMove>>
    {
        let mut checkers = Vec::new();
        if let Some(color) = &self.props.piece_color
        {
            for checker in self.state.checker_pieces.get(&color).unwrap()
            {
                if checker.is_crowned
                {
                    for opponent_checker in self.state.checker_pieces.get(&color.opposite()).unwrap()
                    {
                        let first_position = CheckerPosition { column: checker.position.column + 1,  line: checker.position.line + 1 };
                        if opponent_checker.position == first_position
                        {
                            let second_position = CheckerPosition { column: checker.position.column + 2,  line: checker.position.line + 2 };
                            if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                                !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                                is_allowable_position(&second_position)
                            {
                                checkers.push(
                                    AllowableMove
                                    {
                                        checker_id: checker.id,
                                        captured_piece_position: Some(first_position),
                                        next_position: second_position
                                    })
                            }
                        }
                        let first_position = CheckerPosition { column: checker.position.column - 1,  line: checker.position.line + 1 };
                        if opponent_checker.position == first_position
                        {
                            let second_position = CheckerPosition { column: checker.position.column - 2,  line: checker.position.line + 2 };
                            if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                                !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                                is_allowable_position(&second_position)
                            {
                                checkers.push(
                                    AllowableMove
                                    {
                                        checker_id: checker.id,
                                        captured_piece_position: Some(first_position),
                                        next_position: second_position
                                    })
                            }
                        }
                        let first_position = CheckerPosition { column: checker.position.column - 1,  line: checker.position.line - 1 };
                        if opponent_checker.position == first_position
                        {
                            let second_position = CheckerPosition { column: checker.position.column - 2,  line: checker.position.line - 2 };
                            if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                                !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                                is_allowable_position(&second_position)
                            {
                                checkers.push(
                                    AllowableMove
                                    {
                                        checker_id: checker.id,
                                        captured_piece_position: Some(first_position),
                                        next_position: second_position
                                    })
                            }
                        }
                        let first_position = CheckerPosition { column: checker.position.column + 1,  line: checker.position.line - 1 };
                        if opponent_checker.position == first_position
                        {
                            let second_position = CheckerPosition { column: checker.position.column + 2,  line: checker.position.line - 2 };
                            if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                                !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                                is_allowable_position(&second_position)
                            {
                                checkers.push(
                                    AllowableMove
                                    {
                                        checker_id: checker.id,
                                        captured_piece_position: Some(first_position),
                                        next_position: second_position
                                    })
                            }
                        }

                    }
                }
                else if color == &PieceColor::White
                {
                    for opponent_checker in self.state.checker_pieces.get(&color.opposite()).unwrap()
                    {
                        let first_position = CheckerPosition { column: checker.position.column + 1,  line: checker.position.line + 1 };
                        if opponent_checker.position == first_position
                        {
                            let second_position = CheckerPosition { column: checker.position.column + 2,  line: checker.position.line + 2 };
                            if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                                !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                                is_allowable_position(&second_position)
                            {
                                checkers.push(
                                    AllowableMove
                                    {
                                        checker_id: checker.id,
                                        captured_piece_position: Some(first_position),
                                        next_position: second_position
                                    })
                            }
                        }
                        let first_position = CheckerPosition { column: checker.position.column - 1,  line: checker.position.line + 1 };
                        if opponent_checker.position == first_position
                        {
                            let second_position = CheckerPosition { column: checker.position.column - 2,  line: checker.position.line + 2 };
                            if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                                !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                                is_allowable_position(&second_position)
                            {
                                checkers.push(
                                    AllowableMove
                                    {
                                        checker_id: checker.id,
                                        captured_piece_position: Some(first_position),
                                        next_position: second_position
                                    })
                            }
                        }
                    }
                }
                else
                {
                    for opponent_checker in self.state.checker_pieces.get(&color.opposite()).unwrap()
                    {
                        let first_position = CheckerPosition { column: checker.position.column + 1,  line: checker.position.line - 1 };
                        if opponent_checker.position == first_position
                        {
                            let second_position = CheckerPosition { column: checker.position.column + 2,  line: checker.position.line - 2 };
                            if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                                !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                                is_allowable_position(&second_position)
                            {
                                checkers.push(
                                    AllowableMove
                                    {
                                        checker_id: checker.id,
                                        captured_piece_position: Some(first_position),
                                        next_position: second_position
                                    })
                            }
                        }
                        let first_position = CheckerPosition { column: checker.position.column - 1,  line: checker.position.line - 1 };
                        if opponent_checker.position == first_position
                        {
                            let second_position = CheckerPosition { column: checker.position.column - 2,  line: checker.position.line - 2 };
                            if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                                !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                                is_allowable_position(&second_position)
                            {
                                checkers.push(
                                    AllowableMove
                                    {
                                        checker_id: checker.id,
                                        captured_piece_position: Some(first_position),
                                        next_position: second_position
                                    })
                            }
                        }
                    }
                }


            }
            if !checkers.is_empty()
            {
                Some(checkers)
            }
            else { None }
        }
        else { None }
    }


    fn can_moving_checkers(&self) -> Option<Vec<AllowableMove>>
    {
        let mut checkers = Vec::new();
        if let Some(color) = &self.props.piece_color
        {
            for checker in self.state.checker_pieces.get(&color).unwrap()
            {
                if checker.is_crowned
                {
                    let second_position = CheckerPosition { column: checker.position.column + 1,  line: checker.position.line + 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id,
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }

                    let second_position = CheckerPosition { column: checker.position.column - 1,  line: checker.position.line + 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id,
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }


                    let second_position = CheckerPosition { column: checker.position.column - 1,  line: checker.position.line - 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id,
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                    let second_position = CheckerPosition { column: checker.position.column + 1,  line: checker.position.line - 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id,
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                }
                else if color == &PieceColor::White
                {
                    let second_position = CheckerPosition { column: checker.position.column + 1,  line: checker.position.line + 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id,
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                    let second_position = CheckerPosition { column: checker.position.column - 1,  line: checker.position.line + 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id,
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                }
                else
                {
                    let second_position = CheckerPosition { column: checker.position.column + 1,  line: checker.position.line - 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id,
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                    let second_position = CheckerPosition { column: checker.position.column - 1,  line: checker.position.line - 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id,
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                }
            }
            if !checkers.is_empty()
            {
                Some(checkers)
            }
            else { None }
        }
        else { None }
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
                        match &self.props.piece_color
                        {
                            Some(color) =>
                                {
                                    if self.state.piece_move.len() == 1
                                    {
                                        if self.state.piece_move[0] == (CheckerPosition { column, line })
                                        {
                                            self.state.piece_move = Vec::new();
                                        }
                                        else
                                        {
                                            if let Some(allowable_moves) = &self.state.allowable_moves
                                            {
                                                if let Some(allow_idx) = allowable_moves
                                                    .iter()
                                                    .position(|allowable_move| allowable_move.next_position == CheckerPosition { column, line })
                                                {
                                                    self.state.piece_move.push(CheckerPosition { column, line });

                                                    if let Some(idx) = find_position(
                                                        &self.state.checker_pieces[color],
                                                        &self.state.piece_move[0] )
                                                    {
                                                        self.state.checker_pieces.get_mut(color).unwrap()[idx].position = self.state.piece_move[1].clone();
                                                    }
                                                    if let Some(captured_piece_position) = allowable_moves[allow_idx].captured_piece_position.clone()
                                                    {
                                                        if let Some(idx) = find_position(
                                                            &self.state.checker_pieces[&color.opposite()],
                                                            &captured_piece_position )
                                                        {
                                                            self.state.checker_pieces.get_mut(&color.opposite()).unwrap().remove(idx);
                                                        }

                                                    }
                                                    let data =
                                                        {
                                                            serde_json::to_string(
                                                                &GameData
                                                                {
                                                                    opponent_piece_color: self.props.piece_color.clone().unwrap(),
                                                                    piece_previous_position: self.state.piece_move[0].to_owned(),
                                                                    piece_new_position: self.state.piece_move[1].to_owned(),
                                                                    captured_piece_position: allowable_moves[allow_idx].captured_piece_position.clone(),
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
                                                else { return false; }
                                            }
                                            else { return false; }
                                        }
                                    }
                                    else if let Some(idx) = find_position(
                                        &self.state.checker_pieces[color],
                                        &CheckerPosition { column, line })
                                    {
                                        match self.can_capturing_checkers()
                                        {
                                            Some(capturing_moves) =>
                                                {
                                                    let moves: Vec<AllowableMove> =  capturing_moves
                                                            .into_iter()
                                                            .filter(|allowable_move|
                                                                allowable_move.checker_id == self.state.checker_pieces
                                                                    .get(color).unwrap()[idx].id).collect();
                                                    if !moves.is_empty()
                                                    {
                                                        self.state.allowable_moves = Some(moves);
                                                        self.state.piece_move.push(CheckerPosition { column, line });
                                                    }
                                                    else { return false; }
                                                },
                                            None =>
                                                {
                                                    match self.can_moving_checkers()
                                                    {
                                                        Some(simple_moves) =>
                                                            {
                                                                let moves: Vec<AllowableMove> =  simple_moves
                                                                    .into_iter()
                                                                    .filter(|allowable_move|
                                                                        allowable_move.checker_id == self.state.checker_pieces
                                                                            .get(color).unwrap()[idx].id).collect();
                                                                if !moves.is_empty()
                                                                {
                                                                    self.state.allowable_moves = Some(moves);
                                                                    self.state.piece_move.push(CheckerPosition { column, line });
                                                                }
                                                            },
                                                        None => ()
                                                    }
                                                }
                                        }
                                        yew::services::ConsoleService::log(&format!("{:?}", self.state.allowable_moves));
                                    }
                                    else { return false; }
                                },
                            None => return false,
                        }
                    }
                    else { return false; }
                },
            Msg::LeaveGame =>
                {
                    self.state = State::init();
                    self.props.leave_game.emit(());
                    let request = WsRequest
                        {
                            action: GameAction::SendLeaveGameMessage.as_str(),
                            data: GAME_NAME.to_string()
                        };
                    self.props.send_websocket_data.emit(request);
                    if let Some(user) = &self.props.user
                    {
                        let request_online_users = WsRequest
                            {
                                action: ChatAction::RequestOnlineUsers.as_str(),
                                data: format!("{}", user.user_name)
                            };
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
                    let game_data: GameData = serde_json::from_str(&response.data).unwrap();
                    self.state.is_my_step = !game_data.is_opponent_step;
                    let color =  &game_data.opponent_piece_color;
                    if let Some(idx) = find_position(
                        &self.state.checker_pieces[color],
                        &game_data.piece_previous_position)
                    {
                        self.state.checker_pieces.get_mut(color).unwrap()[idx].position = game_data.piece_new_position;
                    }
                    if let Some(captured_position) = game_data.captured_piece_position
                    {
                        if let Some(idx) = find_position(
                            &self.state.checker_pieces[&color.opposite()],
                            &captured_position)
                        {
                            self.state.checker_pieces.get_mut(&color.opposite()).unwrap().remove(idx);
                        }
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
