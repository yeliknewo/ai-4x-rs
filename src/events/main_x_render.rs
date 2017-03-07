use graphics::NGEncoder;

pub enum MainFromRender<ID>
    where ID: Send
{
    Encoder(NGEncoder, ID),
}

pub enum MainToRender<ID>
    where ID: Send
{
    Encoder(NGEncoder, ID),
}
