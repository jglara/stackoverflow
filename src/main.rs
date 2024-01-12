#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use dotenvy::dotenv;

use persistance::answers_dao;
use sqlx::postgres::PgPoolOptions;

mod cors;
mod handlers;
mod models;
mod persistance;

use cors::*;
use handlers::*;

use persistance::answers_dao::{AnswersDao, AnswersDaoImpl};
use persistance::questions_dao::{QuestionsDao, QuestionsDaoImpl};

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let questions_dao: Box<dyn QuestionsDao + Send + Sync> =
        Box::new(QuestionsDaoImpl::new(pool.clone()));
    let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(AnswersDaoImpl::new(pool));

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
        .manage(questions_dao)
        .manage(answers_dao)
}
