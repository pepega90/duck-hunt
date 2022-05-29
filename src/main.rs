mod dog;
mod burung;

use macroquad::prelude::*;
use macroquad::audio::*;

use dog::*;
use burung::*;

fn config() -> Conf {
    Conf {
        window_title: "Dick hunt".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

fn point_rect_collision(mouse_x: &f32, mouse_y: &f32, rect: &Rect) -> bool {
    if mouse_x >= &rect.x && mouse_x <= &(&rect.x + &rect.w) && mouse_y >= &rect.y && mouse_y <= &(&rect.y + &rect.h) {    
        return true;
    }
    return false;
}

enum Scene {
    MENU,
    PLAY,
}

#[macroquad::main(config)]
async fn main() {
	// load font
	let d = load_ttf_font("texture/duck_hunt.ttf").await.unwrap();
	let f = load_ttf_font("texture/Pixel Font.ttf").await.unwrap();

	// load assets
	let pohon = load_texture("texture/tree.png").await.unwrap();
	let sky_img = load_texture("texture/sky.png").await.unwrap();
    let sky_fail_img = load_texture("texture/sky2.png").await.unwrap();
	let grass_img = load_texture("texture/grass.png").await.unwrap();
	let cloud_img = load_texture("texture/sky3.png").await.unwrap();
	let dog_img = load_texture("texture/dog.png").await.unwrap();
    let crosshair_img = load_texture("texture/crosshair.png").await.unwrap();
    let duck_spritesheet = load_texture("texture/flying_anim.png").await.unwrap();
    let duck_fall_spritesheet = load_texture("texture/fall_anim.png").await.unwrap();

	// load sfx
	let end_round = load_sound("texture/end_round.ogg").await.unwrap();
	let shoot = load_sound("texture/shoot.ogg").await.unwrap();

	let mut dog = Dog {
		img: dog_img,
		pos: Vec2::new(50.,350.),
		frame: Default::default(),
	};

    let mut bird_list : Vec<Bird> = vec![];
    let mut flip = false;
    let mut random_spawn_position = rand::gen_range(84., screen_width() - 84.);
    // show dog
    let mut last_update = get_time();
    let mut count = 3;
    // show start game
    let mut last_show = get_time();
    let mut show_start = 2;
    let mut show = false;
    // timer
    let mut last_time = get_time();
    let mut time = 20;

    // GUI
    let mut score = 0;
    let mut hit = 5;
    let mut current_sky = sky_img;
    // hide cursor
    show_mouse(false);

    // game state
    let mut game_over : bool = false;
    let mut current_scene = Scene::MENU;
    loop {
        clear_background(BLACK);

        let (mouse_x, mouse_y) = mouse_position();

        let crosshair_x = mouse_x - (crosshair_img.width() / 2.);
        let crosshair_y = mouse_y - (crosshair_img.height() / 2.);

        match current_scene {
        	Scene::MENU => 
        	{
        		draw_text_ex("Dick Hunt", 40., screen_height() /2., TextParams{
                 	font: d,
                 	font_size: 150,
                 	color: BLUE,
                 	..Default::default()
                 });

        		if get_time() - last_show > 0.5 && show_start > 0 {
        			last_show = get_time();

        			show_start -= 1;
        		}

    			if show_start == 2
    			{
					draw_text_ex("Press \"Space\" to start", 140., screen_height() /2. + 100., TextParams{
                 	font: f,
                 	font_size: 30,
                 	..Default::default()
                 });
					
    			}

    			if show_start == 0 {
    				show_start = 2;
    			}

    			if is_key_pressed(KeyCode::Space)
    			{
    				current_scene = Scene::PLAY;
    			}
        		

    			draw_text_ex("created by aji mustofa @pepega90", 140., screen_height() - 20., TextParams{
                 	font: f,
                 	font_size: 20,
                 	color: YELLOW,
                 	..Default::default()
                 });
        	} ,
            Scene::PLAY => {
                draw_texture_ex(current_sky, 0., 0., WHITE, DrawTextureParams{
                	dest_size: Some(Vec2::new(800., 600.)),
                	..Default::default()
                });

                draw_texture_ex(cloud_img, 10., -100., WHITE, DrawTextureParams{
                	dest_size: Some(Vec2::new(800., 600.)),
                	..Default::default()
                });

                draw_texture_ex(pohon, 10., -50., WHITE, DrawTextureParams{
                	dest_size: Some(Vec2::new(pohon.width()/2., pohon.height()/2.)),
                	..Default::default()
                });

                

                if is_mouse_button_pressed(MouseButton::Left)
                {
                	play_sound_once(shoot);
                }

            	if get_time() - last_update > 0.8 && count > 0 {
            		last_update = get_time();
            		count -= 1;
            	}

            	if count == 0 {
            		show = false;
            	}

                if show {
            		dog.update();
                }
            	

                while bird_list.len() < 1 && !game_over {
                    flip = !flip;
                    random_spawn_position = rand::gen_range(84., screen_width() - 84.);
                    bird_list.push(Bird {
                        img: duck_spritesheet,
                        fall_img: duck_fall_spritesheet,
                        die: false,
                        pos: Vec2::new(random_spawn_position, screen_height()/2. + 50.),
                        frame: Default::default(),
                        fall_frame: Default::default(),
                        last_update: get_time(),
                        last_fall: get_time(),
                        turn: flip,
                        speed: Vec2::new(1., 1.),
                        rect: Default::default(),
                        fall: false,
                        count: 3,
                    });
                }


                for bird in bird_list.iter_mut() {
                	// cek collision crosshair dengan burung
                	if is_mouse_button_pressed(MouseButton::Left) && point_rect_collision(&mouse_x, &mouse_y, &bird.rect) && current_sky == sky_img {
                		bird.fall = true;
                		score += 10;
                		hit -= 1;
           			} else if is_mouse_button_pressed(MouseButton::Left) && !point_rect_collision(&mouse_x, &mouse_y, &bird.rect) {
                        current_sky = sky_fail_img;
                    }

                    bird.update();

                	// if bird.die {
                	// 	play_sound_once(end_round);
                	// 	dog.pos.x = bird.pos.x;
                	// 	dog.pos.y = 300.;
                	// }

                }

                if let Some(b) = bird_list.first() {
                  if b.pos.y < 10. {
                    current_sky = sky_fail_img;
                  }

                	if b.die {
                		play_sound_once(end_round);
                		dog.pos.x = b.pos.x;
                		dog.pos.y = 300.;
                		show = true;
                		count = 3;
                	}
                }

                bird_list.retain(|b| b.pos.y > -70. && !b.die);

                if bird_list.is_empty()
                {
                    current_sky = sky_img;
                }

                 draw_texture_ex(grass_img, 0., 300., WHITE, DrawTextureParams{
                	dest_size: Some(Vec2::new(screen_width(), screen_height()/2.)),
                	..Default::default()
                });

                 // draw crosshair
                 draw_texture(crosshair_img, crosshair_x, crosshair_y, WHITE);
           		
                 // draw GUI
                 if current_sky == sky_fail_img && !game_over
                 {
                    draw_text_ex("Yahh terbang...", screen_width()/2. - 70., screen_height()/4., TextParams{
                    font: f,
                    font_size: 30,
                    color: BLACK,
                    ..Default::default()
                    });
                 }
                 

                  draw_text_ex("Hit Duck", 200., 530., TextParams{
                 	font: f,
                 	font_size: 20,
                 	..Default::default()
                 });

                  for i in 0..hit 
                  {
                  		draw_texture_ex(duck_spritesheet, 340. + (i as f32) * 84./2., 500., WHITE, DrawTextureParams{
                  			dest_size:Some(Vec2::new(84./2., 84./2.)),
		                    source: Some(Rect::new(1. * duck_spritesheet.width()/3., 0., duck_spritesheet.width()/3., duck_spritesheet.height())),
		                    ..Default::default()
		            }); 
                  }

                  // update timer
                  if get_time() - last_time > 0.7 && time > 0 && !game_over {
                  	last_time = get_time();
                  	time -= 1;
                  }
                  // menapilkan timer
                 draw_text_ex("Time", 73., 515., TextParams{
                 	font: f,
                 	font_size: 20,
                 	..Default::default()
                 });

                 draw_text_ex(format!("{}", time).as_str(), 100., 545., TextParams{
                 	font: f,
                 	font_size: 20,
                 	..Default::default()
                 });

                 // menampilkan score
                 draw_text_ex("Score", 630., 515., TextParams{
                 	font: f,
                 	font_size: 20,
                 	..Default::default()
                 });

                  draw_text_ex(format!("{}", score).as_str(), 666., 545., TextParams{
                 	font: f,
                 	font_size: 20,
                 	..Default::default()
                 });

                  // game over condition
                  if hit == 0 || time == 0 {
                  	game_over = true;
                  }

                  if game_over {
                  	current_sky = sky_fail_img;
              	   	draw_text_ex("Permainan Berakhir", screen_width()/2. - 90., screen_height()/4., TextParams{
                 	font: f,
                 	font_size: 30,
                 	color: BLACK,
                 	..Default::default()
                 });
              	   	draw_text_ex("Tekan \"R\" untuk restart", screen_width()/2. - 60., screen_height()/4. + 50., TextParams{
                 	font: f,
                 	font_size: 20,
                 	color: BLACK,
                 	..Default::default()
                 });
                  }

        	   	if is_key_pressed(KeyCode::R) && game_over {
        	   		game_over = false;
        	   		bird_list.clear();
        	   		time = 20;
        	   		hit = 5;
        	   		score = 0;
        	   		current_sky = sky_img;
          	   	}


           		// helper untuk mengetahui posisi mouse
           		// draw_text(format!("x = {}, y = {},", mouse_x, mouse_y).as_str(), mouse_x, mouse_y, 30.,WHITE);
            },
        }

        next_frame().await
    }
}