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
    fn view_checker(&self, color: &str) -> Html
    {
        html!
        {
            <svg width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="22" cy="22" r="21.75" fill="none" stroke={ color } stroke-width="1.0"/>
            <circle cx="22" cy="22" r="16.75" fill="none" stroke={ color } stroke-width="1.0"/>
            <line x1="22.25" y1="1" x2="22.25" y2="4" stroke={ color } stroke-width="1.0"/>
            <line x1="22.25" y1="40" x2="22.25" y2="43" stroke={ color } stroke-width="1.0"/>
            <line x1="1.36244" y1="18.1072" x2="4.31687" y2="18.6281" stroke={ color } stroke-width="1.0"/>
            <line x1="39.7699" y1="24.8795" x2="42.7244" y2="25.4004" stroke={ color } stroke-width="1.0"/>
            <line x1="2.35195" y1="14.5827" x2="5.17103" y2="15.6087" stroke={ color } stroke-width="1.0"/>
            <line x1="39" y1="27.9214" x2="41.819" y2="28.9475" stroke={ color } stroke-width="1.0"/>
            <line x1="3.93848" y1="11.2835" x2="6.53655" y2="12.7835" stroke={ color } stroke-width="1.0"/>
            <line x1="37.7135" y1="30.7835" x2="40.3115" y2="32.2835" stroke={ color } stroke-width="1.0"/>
            <line x1="6.07375" y1="8.30995" x2="8.37189" y2="10.2383" stroke={ color } stroke-width="1.0"/>
            <line x1="35.9495" y1="33.3787" x2="38.2476" y2="35.307" stroke={ color } stroke-width="1.0"/>
            <line x1="8.69298" y1="5.75237" x2="10.6213" y2="8.05051" stroke={ color } stroke-width="1.0"/>
            <line x1="33.7617" y1="35.6281" x2="35.6901" y2="37.9262" stroke={ color } stroke-width="1.0"/>
            <line x1="11.7165" y1="3.68846" x2="13.2165" y2="6.28654" stroke={ color } stroke-width="1.0"/>
            <line x1="31.2165" y1="37.4635" x2="32.7165" y2="40.0615" stroke={ color } stroke-width="1.0"/>
            <line x1="15.0525" y1="2.18094" x2="16.0786" y2="5.00002" stroke={ color } stroke-width="1.0"/>
            <line x1="28.3913" y1="38.829" x2="29.4173" y2="41.648" stroke={ color } stroke-width="1.0"/>
            <line x1="18.5996" y1="1.27562" x2="19.1205" y2="4.23004" stroke={ color } stroke-width="1.0"/>
            <line x1="25.3719" y1="39.6831" x2="25.8928" y2="42.6375" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(-1 4.37114e-08 4.37114e-08 1 43 22)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(-1 4.37114e-08 4.37114e-08 1 4 22)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(-4.37114e-08 -1 -1 4.37114e-08 22 43)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(-4.37114e-08 -1 -1 4.37114e-08 22 4)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.984808 -0.173648 -0.173648 -0.984808 1.31903 25.6466)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.984808 -0.173648 -0.173648 -0.984808 39.7265 18.8743)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.939693 -0.34202 -0.34202 -0.939693 2.26645 29.1824)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.939693 -0.34202 -0.34202 -0.939693 38.9145 15.8436)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.866025 -0.5 -0.5 -0.866025 3.81348 32.5)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.866025 -0.5 -0.5 -0.866025 37.5885 13)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.766044 -0.642788 -0.642788 -0.766044 5.91306 35.4985)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.766044 -0.642788 -0.642788 -0.766044 35.7888 10.4298)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.642788 -0.766044 -0.766044 -0.642788 8.50146 38.0869)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.642788 -0.766044 -0.766044 -0.642788 33.5702 8.2112)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.5 -0.866025 -0.866025 -0.5 11.5 40.1865)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.5 -0.866025 -0.866025 -0.5 31 6.41154)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.34202 -0.939693 -0.939693 -0.34202 14.8176 41.7336)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.34202 -0.939693 -0.939693 -0.34202 28.1563 5.08554)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.173648 -0.984808 -0.984808 -0.173648 18.3534 42.681)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.173648 -0.984808 -0.984808 -0.173648 25.1257 4.27347)" stroke={ color } stroke-width="1.0"/>
            </svg>
        }
    }


    fn view_crowned_checker(&self, color: &str) -> Html
    {
        html!
        {
            <svg width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
            <circle cx="22" cy="22" r="21.75" fill="none" stroke={ color } stroke-width="1.0"/>
            <circle cx="22" cy="22" r="16.75" fill="none" stroke={ color } stroke-width="1.0"/>
            <line x1="22.25" y1="1" x2="22.25" y2="4" stroke={ color } stroke-width="1.0"/>
            <line x1="22.25" y1="40" x2="22.25" y2="43" stroke={ color } stroke-width="1.0"/>
            <line x1="1.36244" y1="18.1072" x2="4.31687" y2="18.6281" stroke={ color } stroke-width="1.0"/>
            <line x1="39.7699" y1="24.8795" x2="42.7244" y2="25.4004" stroke={ color } stroke-width="1.0"/>
            <line x1="2.35195" y1="14.5827" x2="5.17103" y2="15.6087" stroke={ color } stroke-width="1.0"/>
            <line x1="39" y1="27.9214" x2="41.819" y2="28.9475" stroke={ color } stroke-width="1.0"/>
            <line x1="3.93848" y1="11.2835" x2="6.53655" y2="12.7835" stroke={ color } stroke-width="1.0"/>
            <line x1="37.7135" y1="30.7835" x2="40.3115" y2="32.2835" stroke={ color } stroke-width="1.0"/>
            <line x1="6.07375" y1="8.30995" x2="8.37189" y2="10.2383" stroke={ color } stroke-width="1.0"/>
            <line x1="35.9495" y1="33.3787" x2="38.2476" y2="35.307" stroke={ color } stroke-width="1.0"/>
            <line x1="8.69298" y1="5.75237" x2="10.6213" y2="8.05051" stroke={ color } stroke-width="1.0"/>
            <line x1="33.7617" y1="35.6281" x2="35.69" y2="37.9262" stroke={ color } stroke-width="1.0"/>
            <line x1="11.7165" y1="3.68846" x2="13.2165" y2="6.28654" stroke={ color } stroke-width="1.0"/>
            <line x1="31.2165" y1="37.4635" x2="32.7165" y2="40.0615" stroke={ color } stroke-width="1.0"/>
            <line x1="15.0525" y1="2.18094" x2="16.0786" y2="5.00002" stroke={ color } stroke-width="1.0"/>
            <line x1="28.3913" y1="38.829" x2="29.4173" y2="41.648" stroke={ color } stroke-width="1.0"/>
            <line x1="18.5996" y1="1.27562" x2="19.1205" y2="4.23004" stroke={ color } stroke-width="1.0"/>
            <line x1="25.3719" y1="39.6831" x2="25.8928" y2="42.6375" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(-1 4.37114e-08 4.37114e-08 1 43 22)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(-1 4.37114e-08 4.37114e-08 1 4 22)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(-4.37114e-08 -1 -1 4.37114e-08 22 43)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(-4.37114e-08 -1 -1 4.37114e-08 22 4)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.984808 -0.173648 -0.173648 -0.984808 1.31903 25.6466)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.984808 -0.173648 -0.173648 -0.984808 39.7265 18.8743)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.939693 -0.34202 -0.34202 -0.939693 2.26645 29.1824)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.939693 -0.34202 -0.34202 -0.939693 38.9145 15.8436)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.866025 -0.5 -0.5 -0.866025 3.81348 32.5)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.866025 -0.5 -0.5 -0.866025 37.5885 13)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.766044 -0.642788 -0.642788 -0.766044 5.91306 35.4985)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.766044 -0.642788 -0.642788 -0.766044 35.7888 10.4298)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.642788 -0.766044 -0.766044 -0.642788 8.50146 38.0869)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.642788 -0.766044 -0.766044 -0.642788 33.5702 8.2112)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.5 -0.866025 -0.866025 -0.5 11.5 40.1865)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.5 -0.866025 -0.866025 -0.5 31 6.41154)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.34202 -0.939693 -0.939693 -0.34202 14.8176 41.7336)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.34202 -0.939693 -0.939693 -0.34202 28.1564 5.08554)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.173648 -0.984808 -0.984808 -0.173648 18.3534 42.681)" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="3" y2="-0.25" transform="matrix(0.173648 -0.984808 -0.984808 -0.173648 25.1257 4.27347)" stroke={ color } stroke-width="1.0"/>
            <path d="M28 30C28 29.2044 27.3679 28.4413 26.2426 27.8787C25.1174 27.3161 23.5913 27 22 27C20.4087 27 18.8826 27.3161 17.7574 27.8787C16.6321 28.4413 16 29.2044 16 30" stroke={ color } stroke-width="1.0"/>
            <path d="M16 30L11.7628 18.0791" stroke={ color } stroke-width="1.0"/>
            <line y1="-0.25" x2="12.6491" y2="-0.25" transform="matrix(-0.316227 0.948684 0.948684 0.316227 32 18)" stroke={ color } stroke-width="1.0"/>
            <path d="M20.7588 15.6319C20.8935 16.0021 20.9297 16.4041 20.8653 16.8149C20.8009 17.2256 20.6372 17.6372 20.3834 18.026C20.1296 18.4148 19.7909 18.7733 19.3864 19.0809C18.9819 19.3886 18.5197 19.6394 18.0261 19.8191C17.5325 19.9987 17.0171 20.1037 16.5095 20.128C16.0019 20.1523 15.512 20.0955 15.0677 19.9608C14.6234 19.826 14.2334 19.616 13.92 19.3427C13.6066 19.0695 13.376 18.7383 13.2412 18.3681L13.3983 18.3109C13.5274 18.6656 13.7485 18.983 14.0487 19.2448C14.349 19.5067 14.7227 19.7079 15.1485 19.837C15.5742 19.9661 16.0437 20.0206 16.53 19.9973C17.0164 19.974 17.5102 19.8734 17.9832 19.7012C18.4562 19.5291 18.8991 19.2888 19.2866 18.994C19.6742 18.6992 19.9988 18.3557 20.242 17.9831C20.4851 17.6106 20.6421 17.2162 20.7038 16.8226C20.7655 16.429 20.7308 16.0438 20.6017 15.6891L20.7588 15.6319Z" fill="#C4C4C4" stroke={ color } stroke-width="0.25"/>
            <path d="M23.2413 15.6319C23.1065 16.0021 23.0703 16.4041 23.1347 16.8149C23.1992 17.2256 23.3629 17.6372 23.6167 18.026C23.8704 18.4148 24.2092 18.7733 24.6137 19.0809C25.0181 19.3886 25.4804 19.6394 25.974 19.8191C26.4676 19.9987 26.9829 20.1037 27.4905 20.128C27.9981 20.1523 28.4881 20.0955 28.9324 19.9608C29.3767 19.826 29.7667 19.616 30.08 19.3427C30.3934 19.0695 30.6241 18.7383 30.7588 18.3681L30.6017 18.3109C30.4726 18.6656 30.2516 18.983 29.9513 19.2448C29.651 19.5067 29.2774 19.7079 28.8516 19.837C28.4259 19.9661 27.9564 20.0206 27.47 19.9973C26.9836 19.974 26.4899 19.8734 26.0169 19.7012C25.5439 19.5291 25.101 19.2888 24.7134 18.994C24.3258 18.6992 24.0012 18.3557 23.7581 17.9831C23.5149 17.6106 23.358 17.2162 23.2963 16.8226C23.2346 16.429 23.2693 16.0438 23.3984 15.6891L23.2413 15.6319Z" fill="#C4C4C4" stroke={ color } stroke-width="0.25"/>
            <circle r="1.25" transform="matrix(1 0 0 -1 12.5 17.5)" fill="none" stroke={ color } stroke-width="1.0"/>
            <circle r="1.25" transform="matrix(1 0 0 -1 31.5 17.5)" fill="none" stroke={ color } stroke-width="1.0"/>
            <circle r="1.75" transform="matrix(1 0 0 -1 22 15)" fill="none" stroke={ color } stroke-width="1.0"/>
            </svg>
        }
    }





    fn view_black_cells(&self, column: usize, line: usize) -> Html
    {
        let white_checker: Html =
            {
                if let Some(color) = &self.props.piece_color
                {
                    match color
                    {
                        PieceColor::White => html! { <div class="checker_my_white">{ self.view_checker("black") }</div> },
                        PieceColor::Black => html! { <div class="checker_white">{ self.view_checker("black") }</div> },
                    }
                }
                else { html! { <div class="checker_white">{ self.view_checker("black") }</div> } }
            };
        let white_checker_active: Html = html! { <div class="checker_my_white_active">{ self.view_checker("black") }</div> };
        let white_crowned_checker: Html =
            {
                if let Some(color) = &self.props.piece_color
                {
                    match color
                    {
                        PieceColor::White => html! { <div class="checker_my_white">{ self.view_crowned_checker("black") }</div> },
                        PieceColor::Black => html! { <div class="checker_white">{ self.view_crowned_checker("black") }</div> },
                    }
                }
                else { html! { <div class="checker_white">{ self.view_crowned_checker("black") }</div> } }
            };
        let white_crowned_checker_active: Html = html! { <div class="checker_my_white_active">{ self.view_crowned_checker("black") }</div> };
        let black_checker: Html =
            {
                if let Some(color) = &self.props.piece_color
                {
                    match color
                    {
                        PieceColor::White => html! { <div class="checker_black">{ self.view_checker("white") }</div> },
                        PieceColor::Black => html! { <div class="checker_my_black">{ self.view_checker("white") }</div> },
                    }
                }
                else { html! { <div class="checker_black">{ self.view_checker("white") }</div> } }
            };
        let black_checker_active: Html = html! { <div class="checker_my_black_active">{ self.view_checker("white") }</div> };
        let black_crowned_checker: Html =
            {
                if let Some(color) = &self.props.piece_color
                {
                    match color
                    {
                        PieceColor::White => html! { <div class="checker_black">{ self.view_crowned_checker("white") }</div> },
                        PieceColor::Black => html! { <div class="checker_my_black">{ self.view_crowned_checker("white") }</div> },
                    }
                }
                else { html! { <div class="checker_black">{ self.view_crowned_checker("white") }</div> } }
            };
        let black_crowned_checker_active: Html = html! { <div class="checker_my_black_active">{ self.view_crowned_checker("white") }</div> };
        html!
        {
            if let Some(idx) = self.state.checker_pieces[&PieceColor::White]
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
                                if self.state.checker_pieces[&PieceColor::White][idx].is_crowned
                                {
                                    white_crowned_checker_active.clone()
                                }
                                else
                                {
                                    white_checker_active.clone()
                                }
                            }
                            else
                            {
                                if self.state.checker_pieces[&PieceColor::White][idx].is_crowned
                                {
                                    white_crowned_checker.clone()
                                }
                                else
                                {
                                    white_checker.clone()
                                }
                            }
                        }
                    </div>
                }
            }
            else if let Some(idx) = self.state.checker_pieces[&PieceColor::Black]
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
                                if self.state.checker_pieces[&PieceColor::Black][idx].is_crowned
                                {
                                    black_crowned_checker_active.clone()
                                }
                                else
                                {
                                    black_checker_active.clone()
                                }
                            }
                            else
                            {
                                if self.state.checker_pieces[&PieceColor::Black][idx].is_crowned
                                {
                                    black_crowned_checker.clone()
                                }
                                else
                                {
                                    black_checker.clone()
                                }
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
                            <div class="checker_board_header_container">
                                <p>{ "You win !!!" }</p>
                                <button class="checker_board_header_button"
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
                            <div class="checker_board_header_container">
                                <p>{ "You lose :-(" }</p>
                                <button class="checker_board_header_button"
                                    disabled=!self.props.is_in_game
                                    onclick=self.link.callback(|_| Msg::LeaveGame)>
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
                <div class="checker_board_header_container">
                    {
                        if self.state.is_my_step
                        {
                            html! { <p> { "Your move" } </p> }
                        }
                        else if self.props.is_in_game && !self.state.is_my_step && !self.state.is_steps_order_defined
                        {
                            if let Some(color) = &self.props.piece_color
                            {
                                match color
                                {
                                    PieceColor::White => html! { <p> { "Your move" } </p> },
                                    PieceColor::Black => html! { <p> { "Opponent's move" } </p> },
                                }
                            }
                            else { html! { <p> { "Opponent's move" } </p> } }
                        }
                        else if !self.props.is_in_game
                        {
                            html! { <p> { "Waiting new game" } </p> }
                        }
                        else
                        {
                            html! { <p> { "Opponent's move" } </p> }
                        }
                    }
                    <button class="checker_board_header_button"
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
                for opponent_checker in self.state.checker_pieces.get(&color.opposite()).unwrap()
                {
                    let first_position = CheckerPosition { column: &checker.position.column + 1,  line: &checker.position.line + 1 };
                    if opponent_checker.position == first_position
                    {
                        let second_position = CheckerPosition { column: &checker.position.column + 2,  line: &checker.position.line + 2 };
                        if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                            !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                            is_allowable_position(&second_position)
                        {
                            checkers.push(
                                AllowableMove
                                {
                                    checker_id: checker.id.clone(),
                                    captured_piece_position: Some(first_position),
                                    next_position: second_position
                                })
                        }
                    }
                    let first_position = CheckerPosition { column: &checker.position.column - 1,  line: &checker.position.line + 1 };
                    if opponent_checker.position == first_position
                    {
                        let second_position = CheckerPosition { column: &checker.position.column - 2,  line: &checker.position.line + 2 };
                        if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                            !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                            is_allowable_position(&second_position)
                        {
                            checkers.push(
                                AllowableMove
                                {
                                    checker_id: checker.id.clone(),
                                    captured_piece_position: Some(first_position),
                                    next_position: second_position
                                })
                        }
                    }
                    let first_position = CheckerPosition { column: &checker.position.column - 1,  line: &checker.position.line - 1 };
                    if opponent_checker.position == first_position
                    {
                        let second_position = CheckerPosition { column: &checker.position.column - 2,  line: &checker.position.line - 2 };
                        if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                            !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                            is_allowable_position(&second_position)
                        {
                            checkers.push(
                                AllowableMove
                                {
                                    checker_id: checker.id.clone(),
                                    captured_piece_position: Some(first_position),
                                    next_position: second_position
                                })
                        }
                    }
                    let first_position = CheckerPosition { column: &checker.position.column + 1,  line: &checker.position.line - 1 };
                    if opponent_checker.position == first_position
                    {
                        let second_position = CheckerPosition { column: &checker.position.column + 2,  line: &checker.position.line - 2 };
                        if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                            !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                            is_allowable_position(&second_position)
                        {
                            checkers.push(
                                AllowableMove
                                {
                                    checker_id: checker.id.clone(),
                                    captured_piece_position: Some(first_position),
                                    next_position: second_position
                                })
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
                    let second_position = CheckerPosition { column: &checker.position.column + 1,  line: &checker.position.line + 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id.clone(),
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }

                    let second_position = CheckerPosition { column: &checker.position.column - 1,  line: &checker.position.line + 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id.clone(),
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }


                    let second_position = CheckerPosition { column: &checker.position.column - 1,  line: &checker.position.line - 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id.clone(),
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                    let second_position = CheckerPosition { column: &checker.position.column + 1,  line: &checker.position.line - 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id.clone(),
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                }
                else if color == &PieceColor::White
                {
                    let second_position = CheckerPosition { column: &checker.position.column + 1,  line: &checker.position.line + 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id.clone(),
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                    let second_position = CheckerPosition { column: &checker.position.column - 1,  line: &checker.position.line + 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id.clone(),
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                }
                else
                {
                    let second_position = CheckerPosition { column: &checker.position.column + 1,  line: &checker.position.line - 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id.clone(),
                                captured_piece_position: None,
                                next_position: second_position
                            })
                    }
                    let second_position = CheckerPosition { column: &checker.position.column - 1,  line: &checker.position.line - 1 };
                    if !self.state.checker_pieces.get(&color).unwrap().iter().any(|piece| piece.position == second_position) &&
                        !self.state.checker_pieces.get(&color.opposite()).unwrap().iter().any(|piece| piece.position == second_position) &&
                        is_allowable_position(&second_position)
                    {
                        checkers.push(
                            AllowableMove
                            {
                                checker_id: checker.id.clone(),
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
                if !&self.state.is_steps_order_defined
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
                                        if self.state.piece_move[0] == (CheckerPosition { column: column.clone(), line: line.clone() })
                                        {
                                            self.state.piece_move = Vec::new();
                                        }
                                        else
                                        {
                                            if let Some(allowable_moves) = &self.state.allowable_moves
                                            {
                                                if let Some(allow_idx) = allowable_moves
                                                    .iter()
                                                    .position(|allowable_move| allowable_move.next_position ==
                                                        CheckerPosition { column: column.clone(), line: line.clone() })
                                                {
                                                    self.state.piece_move.push(CheckerPosition { column: column.clone(), line: line.clone() });

                                                    if let Some(idx) = find_position(
                                                        &self.state.checker_pieces[color],
                                                        &self.state.piece_move[0] )
                                                    {

                                                        self.state.checker_pieces.get_mut(color).unwrap()[idx].position = self.state.piece_move[1].clone();
                                                        match color
                                                        {
                                                            PieceColor::White =>
                                                                {
                                                                    if self.state.piece_move[1].line == 8
                                                                    {
                                                                        self.state.checker_pieces.get_mut(color).unwrap()[idx.clone()].is_crowned = true;
                                                                    }
                                                                },
                                                            PieceColor::Black =>
                                                                {
                                                                    if self.state.piece_move[1].line == 1
                                                                    {
                                                                        self.state.checker_pieces.get_mut(color).unwrap()[idx.clone()].is_crowned = true;
                                                                    }
                                                                }
                                                        }
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
                                                    self.state.is_my_step =
                                                        {
                                                            if let Some(capturing_moves) = self.can_capturing_checkers()
                                                            {
                                                                if let Some(idx) = capturing_moves
                                                                    .iter()
                                                                    .position(|allowable_move| allowable_move.checker_id == allowable_moves[allow_idx].checker_id)
                                                                {
                                                                    if let Some(allowable_moves) = &self.state.allowable_moves
                                                                    {
                                                                        if allowable_moves
                                                                            .iter()
                                                                            .all(|allowable_move|
                                                                                {
                                                                                    if let Some(_) = allowable_move.captured_piece_position
                                                                                    {
                                                                                        true
                                                                                    }
                                                                                    else { false }
                                                                                })
                                                                        {
                                                                            true
                                                                        }
                                                                        else { false }
                                                                    }
                                                                    else { false }
                                                                }
                                                                else { false }
                                                            }
                                                            else { false }
                                                        };

                                                    let data =
                                                        {
                                                            serde_json::to_string(
                                                                &GameData
                                                                {
                                                                    opponent_piece_color: self.props.piece_color.clone().unwrap(),
                                                                    piece_previous_position: self.state.piece_move[0].to_owned(),
                                                                    piece_new_position: self.state.piece_move[1].to_owned(),
                                                                    captured_piece_position: allowable_moves[allow_idx].captured_piece_position.clone(),
                                                                    is_opponent_step: self.state.is_my_step.clone(),
                                                                }).unwrap()
                                                        };
                                                    let request = WsRequest
                                                    {
                                                        action: GameAction::SendCheckerPieceMove.as_str(),
                                                        data
                                                    };
                                                    self.props.send_websocket_data.emit(request);
                                                    self.state.piece_move = Vec::new();
                                                    if self.state.checker_pieces[&color.opposite()].is_empty()
                                                    {
                                                        self.state.game_result = Some(GameResult::Win);
                                                    }
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
                                                        self.state.piece_move.push(CheckerPosition { column: column.clone(), line: line.clone() });
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
                                                                    self.state.piece_move.push(CheckerPosition { column: column.clone(), line: line.clone() });
                                                                }
                                                            },
                                                        None => ()
                                                    }
                                                }
                                        }
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
                        self.state.checker_pieces.get_mut(color).unwrap()[idx].position = game_data.piece_new_position.clone();
                        match color
                        {
                            PieceColor::White =>
                                {
                                    if game_data.piece_new_position.line == 8
                                    {
                                        self.state.checker_pieces.get_mut(color).unwrap()[idx.clone()].is_crowned = true;
                                    }
                                },
                            PieceColor::Black =>
                                {
                                    if game_data.piece_new_position.line == 1
                                    {
                                        self.state.checker_pieces.get_mut(color).unwrap()[idx.clone()].is_crowned = true;
                                    }
                                }
                        }
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
                    if self.state.checker_pieces[&color.opposite()].is_empty()
                    {
                        self.state.game_result = Some(GameResult::Lose);
                    }
                }
                else if response.action == GameAction::ReceivedLeaveGameMessage.as_str()
                {
                    if let None = &self.state.game_result
                    {
                        self.state.game_result = Some(GameResult::Win);
                    }
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
