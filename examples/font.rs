use nae::prelude::*;

#[nae_start]
fn main() {
    nae::with_state(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> (Font, Font) {
    (Font::default(), app.load_file("../assets/UbuntuMono-R.ttf").unwrap())
}

fn draw(app: &mut App, font: &mut (Font, Font)) {
    let draw = app.draw();
    draw.begin();
    draw.clear(rgba(0.1, 0.2, 0.3, 1.0));
    draw.set_color(Color::Red);
//    draw.transform().skew(0.1, 1.0);
    draw.text(&font.1, r#"Lorem ipsum dolor sit amet, ferri simul omittantur eam eu, no debet doming dolorem ius. Iriure vocibus est te, natum delicata dignissim pri ea. Purto docendi definitiones no qui. Vel ridens instructior ad, vidisse percipitur et eos. Alienum ocurreret laboramus mei cu, usu ne meliore nostrum, usu tritani luptatum electram ad.
Vis oratio tantas prodesset et, id stet inermis mea, at his copiosae accusata. Mel diam accusata argumentum cu, ut agam consul invidunt est. Ocurreret appellantur deterruisset no vis, his alia postulant inciderint no. Has albucius offendit at. An has noluisse comprehensam, vel veri dicit blandit ea, per paulo noluisse reformidans no. Nec ad sale illum soleat, agam scriptorem ad per.
An cum odio mucius apeirian, labores conceptam ex nec, eruditi habemus qualisque eam an. Eu facilisi maluisset eos, fabulas apeirian ut qui, no atqui blandit vix. Apeirian phaedrum pri ex, vel hinc omnes sapientem et, vim vocibus legendos disputando ne. Et vel semper nominati rationibus, eum lorem causae scripta no.
Ut quo elitr viderer constituam, pro omnesque forensibus at. Timeam scaevola mediocrem ut pri, te pro congue delicatissimi. Mei wisi nostro imperdiet ea, ridens salutatus per no, ut viris partem disputationi sit. Exerci eripuit referrentur vix at, sale mediocrem repudiare per te, modus admodum an eam. No vocent indoctum vis, ne quodsi patrioque vix. Vocent labores omittam et usu.
Democritum signiferumque id nam, enim idque facilis at his. Inermis percipitur scriptorem sea cu, est ne error ludus option. Graecis expetenda contentiones cum et, ius nullam impetus suscipit ex. Modus clita corrumpit mel te, qui at lorem harum, primis cetero habemus sea id. Ei mutat affert dolorum duo, eum dissentias voluptatibus te, libris theophrastus duo id.
    "#, 0.0, 0.0, 20.0);
//    draw.transform().pop();
    draw.set_color(Color::Green);
    draw.text(&font.0, "Hello world! This is hard...", 0.0, 120.0, 140.0);
    draw.set_color(rgba(0.4, 0.7, 0.7, 1.0));
    draw.text(
        &font.0,
        "Ok this is a super mega text that I want to do really well.. 3!·$%&/()=?",
        100.0,
        480.0,
        140.0,
    );
    draw.end();
}
