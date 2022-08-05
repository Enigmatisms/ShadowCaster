use casting::caster::viz;

fn main() {
    nannou::app(viz::model).update(viz::update).run();
}