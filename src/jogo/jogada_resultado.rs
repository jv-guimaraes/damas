use super::jogador::Jogador;

#[derive(Debug)]
pub enum JogadaResultado {
    Falha,          // Jogada invalida. Não passa o turno nem mexe no tabuleiro
    Sucesso,        // Jogada válida e passa o turno. Não tem mais possiveis captura
    Sequencia,      // Jogada válida e não passa o turno. Ainda tem pessas para capturar
    FimDoJogo(Jogador), // Jogada válida e fim do jogo. Retorna a vez de quem ganhou
}