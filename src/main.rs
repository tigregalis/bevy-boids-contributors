#![feature(async_closure)]
use bevy::{prelude::*, render::camera::Camera, tasks::IoTaskPool};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
struct GithubContributor {
    total: u32,
    weeks: Vec<GithubWeek>,
    author: GithubAuthor,
}

#[derive(Serialize, Deserialize, Debug)]
struct GithubWeek {
    // w: u32,
    a: u32,
    d: u32,
    // c: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct GithubAuthor {
    login: String,
    // id: u32,
    // node_id: String,
    avatar_url: String,
    // gravatar_id: String,
    // url: String,
    // html_url: String,
    // followers_url: String,
    // following_url: String,
    // gists_url: String,
    // starred_url: String,
    // subscriptions_url: String,
    // organizations_url: String,
    // repos_url: String,
    // events_url: String,
    // received_events_url: String,
    // r#type: String,
    // site_admin: bool,
}

struct Contribution {
    name: String,
    avatar: String,
    commits: u32,
    additions: u32,
    deletions: u32,
}

impl GithubContributor {
    fn get_contributors(json_data: &str) -> serde_json::Result<Vec<GithubContributor>> {
        serde_json::from_str(json_data)
    }
}

fn get_contributions(github_contributors: Vec<GithubContributor>) -> Vec<Contribution> {
    let mut contributions: Vec<Contribution> = github_contributors
        .into_iter()
        .map(|contributor| {
            let name = contributor.author.login;
            let avatar = contributor.author.avatar_url;
            let commits = contributor.total;
            let (additions, deletions) = contributor
                .weeks
                .iter()
                .fold((0, 0), |(a, d), week| (a + week.a, d + week.d));
            Contribution {
                name,
                avatar,
                commits,
                additions,
                deletions,
            }
        })
        .collect();
    contributions.sort_by(|a, b| (b.additions + b.deletions).cmp(&(a.additions + a.deletions)));

    contributions
}

// async fn foo() -> u8 {
//     reqwest::get();
//     5
// }

async fn foo() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(resp)
}

use async_compat::{Compat, CompatExt};

// fn main() {
//     Compat::new(async {
//         // Make an HTTP GET request.
//         let response = reqwest::get("https://www.rust-lang.org").await.unwrap();
//         println!("{}", response.text().await.unwrap());

//         // Start an HTTP server.
//         let routes = warp::any().map(|| "Hello from warp!");
//         warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
//     });
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let https = HttpsConnector::new();
    // let client = Client::builder().build::<_, Body>(https);

    // let req = Request::builder()
    //     .uri("https://api.github.com/repos/bevyengine/bevy/stats/contributors")
    //     .header("User-Agent", "tigregalis/bevy-contributors-list")
    //     .body(Body::empty())?;

    // let res = client.request(req).await?;

    // println!("Response: {}", res.status());

    // let bytes = hyper::body::to_bytes(res.into_body()).await?;
    // let data = String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8");

    // let contributors = GithubContributor::get_contributors(data.as_str())?;

    // let contributions = get_contributions(contributors);let task_pool = app

    let mut app = App::build();

    let (tx, rx) = std::sync::mpsc::channel();

    app.add_plugins(DefaultPlugins)
        // .add_startup_system(setup.system())
        .add_resource(rx)
        .add_startup_system(io_task_pool_debug.system())
        .add_system(
            (|| {
                println!("hello");
            })
            .system(),
        );

    // .add_system(birds_thinking.system())
    // .add_system(birds_flying.system())
    // .add_system(birds_turning.system())
    // .add_system(watch_birds.system())
    // .add_resource(contributions);

    // let task_pool = app
    //     .resources()
    //     .get::<IoTaskPool>()
    //     .expect("IoTaskPool resource not found")
    //     .0
    //     .clone();

    // dbg!(task_pool);

    app.run();

    Ok(())
}

struct Bird {
    name: String,
    cohesion_range: f32,
    alignment_range: f32,
    separation_range: f32,
    velocity: Vec3,
    size: f32,
}

#[derive(Default, Debug)]
struct Cohesion(Vec3);

#[derive(Default, Debug)]
struct Alignment(Vec3);

#[derive(Default, Debug)]
struct Separation(Vec3);

struct MainCamera;

struct Watch(Option<Entity>);

fn setup(
    mut commands: Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    contributions: Res<Vec<Contribution>>,
) {
    commands
        .spawn(Camera2dComponents::default())
        .with(MainCamera);
    let count = contributions.len();
    let columns = (count as f32).sqrt().ceil() as usize;
    let mut watch = None;
    for (index, contribution) in contributions.iter().enumerate() {
        println!(
            "{} changed {} lines of code across {} commits",
            contribution.name,
            contribution.additions + contribution.deletions,
            contribution.commits,
        );
        let loc = (contribution.additions + contribution.deletions) as f32;
        let size = (loc.ln() + 1.0).sqrt() * 10.0;
        let color = Color::rgba(
            index as f32 / count as f32,
            1.0 - index as f32 / count as f32,
            1.25 - 0.5 * index as f32 / count as f32,
            0.5,
        );
        println!("size of \"{}\" is {}", contribution.name, size);
        println!("row {}, column {}", index / columns, index % columns);
        let bird = commands
            .spawn(SpriteComponents {
                material: color_materials.add(color.into()),
                sprite: Sprite {
                    size: Vec2::new(size, size / 2.0),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(
                    (index / columns) as f32 * 40.0,
                    (index % columns) as f32 * 40.0,
                    index as f32,
                )),
                ..Default::default()
            })
            .with(Bird {
                name: contribution.name.clone(),
                cohesion_range: 200.0,
                alignment_range: 100.0,
                separation_range: 50.0,
                velocity: {
                    let add_even = contribution.additions % 2 == 0;
                    let del_even = contribution.deletions % 2 == 0;
                    let add = contribution.additions as f32;
                    let del = contribution.deletions as f32;
                    let x = if add_even { -add } else { add };
                    let y = if del_even { -del } else { del };
                    Vec3::new(x as f32, y as f32, 0.0).normalize() * 10.0
                },
                size,
            })
            .with_bundle((
                Cohesion::default(),
                Alignment::default(),
                Separation::default(),
            ))
            .current_entity();
        if let None = watch {
            watch = bird;
        }
        // .with_children(|parent| parent.spawn(()));
    }
    commands.insert_resource(Watch(watch));

    println!("number of contributors is {}", contributions.len());
}

fn io_task_pool_debug(commands: Commands, time: Res<Time>, io_task_pool: Res<IoTaskPool>) {
    let (tx, rx) = std::sync::mpsc::channel();
    commands.insert_resource(rx);
    // dbg!(&io_task_pool.0);
    // io_task_pool.spawn(async {
    //     let num = foo().await;
    //     println!("hello {}", num);
    // });
    dbg!(time.seconds_since_startup);
    // io_task_pool.scope(|scope| {
    //     scope.spawn(Compat::new(async {
    //         println!("hello");
    //         dbg!(time.seconds_since_startup);
    //         let num = foo().await;
    //         println!("hello {:?}", num);
    //         dbg!(time.seconds_since_startup);
    //     }));
    // });
    io_task_pool
        .spawn(Compat::new(async move {
            println!("hello");
            // dbg!(time.seconds_since_startup);
            let num = foo().await;
            println!("hello {:?}", num);
            tx.send(num);
            // dbg!(time.seconds_since_startup);
        }))
        .detach();
    // dbg!(&io_task_pool.0);
}

fn birds_thinking(
    watch: Res<Watch>,
    mut q0: Query<(
        Entity,
        &Bird,
        &Transform,
        &mut Cohesion,
        &mut Alignment,
        &mut Separation,
    )>,
    q1: Query<(Entity, &Bird, &Transform)>,
) {
    for (entity, bird, transform, mut cohesion, mut alignment, mut separation) in q0.iter_mut() {
        let mut cohesion_neighbours = Vec::new();

        let mut alignment_neighbours = Vec::new();

        let mut separation_neighbours = Vec::new();

        for (entity_other, bird_other, transform_other) in q1.iter() {
            if entity != entity_other {
                let distance = Vec2::length(
                    transform.translation.truncate() - transform_other.translation.truncate(),
                );

                // cohesion
                if distance <= bird.cohesion_range {
                    cohesion_neighbours.push(transform_other.translation.truncate().extend(0.0));
                }

                // alignment
                if distance <= bird.alignment_range {
                    alignment_neighbours.push(bird_other.velocity);
                }

                // separation
                if 0.0 < distance && distance <= bird.separation_range {
                    let towards_me =
                        transform.translation.truncate() - transform_other.translation.truncate();
                    separation_neighbours
                        .push(towards_me.normalize().extend(0.0) / towards_me.length().powf(2.0));
                }
            }
        }

        // calculate cohesion vector
        // the bird is drawn to the centre of its neighbours (average of positions)
        let cohesion_centre = if cohesion_neighbours.len() > 0 {
            cohesion_neighbours
                .iter()
                .fold(Vec3::zero(), |total, current| total + *current)
                / cohesion_neighbours.len() as f32
        } else {
            Vec3::zero()
        };
        let cohesion_new = cohesion_centre - transform.translation.truncate().extend(0.0);

        // calculate alignment vector
        // the bird moves in a similar direction to that of its neighbours (average of velocities)
        let alignment_new = if alignment_neighbours.len() > 0 {
            alignment_neighbours
                .iter()
                .fold(Vec3::zero(), |total, current| total + *current)
                / alignment_neighbours.len() as f32
        } else {
            Vec3::zero()
        };

        // calculate separation vector
        // the bird tries to stay apart from its neighbours (average of relative positions)
        let separation_new = if separation_neighbours.len() > 0 {
            separation_neighbours
                .iter()
                .fold(Vec3::zero(), |total, current| total + *current)
                / separation_neighbours.len() as f32
        } else {
            Vec3::zero()
        };
        // reposition the bird
        let cohesion_new = if cohesion_new.length() > 0.0 {
            cohesion_new.normalize()
        } else {
            cohesion_new
        };
        let alignment_new = if alignment_new.length() > 0.0 {
            alignment_new.normalize()
        } else {
            alignment_new
        };
        let separation_new = if separation_new.length() > 0.0 {
            separation_new.normalize()
        } else {
            separation_new
        };

        // if let Some(e) = watch.0 {
        //     if e == entity {
        //         dbg!(&cohesion_new, &alignment_new, &separation_new);
        //     }
        // }

        cohesion.0 = cohesion_new;
        alignment.0 = alignment_new;
        separation.0 = separation_new;
    }
}

fn birds_flying(
    time: Res<Time>,
    watch: Res<Watch>,
    mut q0: Query<(
        Entity,
        &mut Bird,
        &mut Transform,
        &Cohesion,
        &Alignment,
        &Separation,
    )>,
) {
    const MAX_VELOCITY: f32 = 50.0;
    const MAX_ACCELERATION: f32 = 1000.0;
    for (entity, mut bird, mut transform, cohesion, alignment, separation) in q0.iter_mut() {
        let mut acceleration = 5.0 * cohesion.0 + 10.0 * alignment.0 + 2.0 * separation.0;

        if acceleration.length() > 0.0 {
            acceleration = acceleration.normalize() * acceleration.length().min(MAX_ACCELERATION);
        }

        // if let Some(e) = watch.0 {
        //     if e == entity {
        //         dbg!(&acceleration);
        //     }
        // }

        bird.velocity += acceleration * time.delta_seconds;

        if bird.velocity.length() > 0.0 {
            bird.velocity = bird.velocity.normalize() * bird.velocity.length().min(MAX_VELOCITY);
        }

        // if let Some(e) = watch.0 {
        //     if e == entity {
        //         dbg!(&bird.velocity);
        //     }
        // }

        transform.translation += bird.velocity * time.delta_seconds;
    }
}

fn birds_turning(mut q0: Query<(&Bird, &mut Transform)>) {
    for (bird, mut transform) in q0.iter_mut() {
        transform.rotation = Quat::from_rotation_z((bird.velocity.y() / bird.velocity.x()).atan());
    }
}

fn watch_birds(
    time: Res<Time>,
    watch: Res<Watch>,
    keyboard_input: Res<Input<KeyCode>>,
    q0: Query<(&Bird, &Transform)>,
    mut q1: Query<(&MainCamera, &Camera, &mut Transform)>,
) {
    let translation = &mut q1.iter_mut().next().unwrap().2.translation;

    let dt = time.delta_seconds;

    // if let Some(e) = watch.0 {
    //     if let Ok((_, transform)) = q0.get(e) {
    //         translation.set_x(transform.translation.x());
    //         translation.set_y(transform.translation.y());
    //     }
    // }

    // let (total, count) = q0
    //     .iter()
    //     .map(|(_, transform)| transform.translation)
    //     .fold((Vec3::zero(), 0.0), |(total, count), current| {
    //         (total + current, count + 1.0)
    //     });
    // let centre = total / count;
    // translation.set_x(centre.x());
    // translation.set_y(centre.y());

    if keyboard_input.pressed(KeyCode::Left) {
        *translation.x_mut() -= 100.0 * dt;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        *translation.x_mut() += 100.0 * dt;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        *translation.y_mut() += 100.0 * dt;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        *translation.y_mut() -= 100.0 * dt;
    }
}
