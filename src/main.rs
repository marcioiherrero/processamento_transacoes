use std::io::{BufReader, BufRead};

#[derive(Debug)]
enum ErroTransacao {
  ArquivoNaoEncontrado(String),
  LinhaInvalida(usize, String),
  ValorInvalido(usize, String),
  TipoInvalido(usize, String),
  DataInvalida(usize, String),
  Erro(String),
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Transacao {
  data: String,
  tipo: String,
  valor: i64,
  descricao: String,
}

fn parse_linha(linha: &str, num_linha: usize) -> Result<Transacao, ErroTransacao> {
  let dados: Vec<&str> = linha.split(',').collect();
  
  let data = dados.get(0).ok_or(ErroTransacao::DataInvalida(num_linha, linha.to_string()))?;
  let tipo = dados.get(1).ok_or(ErroTransacao::TipoInvalido(num_linha, linha.to_string()))?;
  let valor = dados.get(2).ok_or(ErroTransacao::ValorInvalido(num_linha, linha.to_string()))?;
  let valor = valor.replace('.', "").parse::<i64>().map_err(|_| ErroTransacao::ValorInvalido(num_linha, linha.to_string()))?;
  let descricao = dados.get(3).ok_or(ErroTransacao::LinhaInvalida(num_linha, linha.to_string()))?;
  
  let trs = Transacao {
    data: data.to_string(),
    tipo: tipo.to_string(),
    valor: valor,
    descricao: descricao.to_string(),
  };
  
  Ok(trs)
}

fn ler_transacoes(caminho: &str) -> Result<Vec<Transacao>, ErroTransacao> {
  let mut vec_trs = vec![];
  
  let arquivo = std::fs::File::open(caminho)
    .map_err(|_| ErroTransacao::Erro("Erro na leitura".to_string()))?;
  
  let reader = BufReader::new(arquivo);
  
  for (linha, dado) in reader.lines().enumerate() {
    if linha == 0 {
      continue;
    }
    
    let texto = dado.unwrap();
    
    let v = parse_linha(&texto, linha);
    
    if let Err(erro) = &v {
      let trs_erro = Transacao {
        data: String::new(),
        tipo: String::from("Erro"),
        valor: 0,
        descricao: match erro {
          _ => format!("{:?}", erro),
        },
      };
      vec_trs.push(trs_erro);
    }
    else {
      vec_trs.push(v.unwrap());
    }
  }
  
  gerar_relatorio(&vec_trs);
  
  Ok(vec_trs)
}

fn gerar_relatorio(transacoes: &[Transacao]) -> Result<(), ErroTransacao> {
  let v_debito: i64 = transacoes.iter()
    .filter(|x| x.tipo == "debito")
    .map(|x| x.valor)
    .sum();
  
  let v_credito: i64 = transacoes.iter()
    .filter(|x| x.tipo == "credito")
    .map(|x| x.valor)
    .sum();
  
  let qtd_debito = transacoes.iter()
    .filter(|x| x.tipo == "debito")
    .count();
  
  let qtd_credito = transacoes.iter()
    .filter(|x| x.tipo == "credito")
    .count();
  
  let max_debito = transacoes.iter()
    .filter(|x| x.tipo == "debito")
    .map(|x| x.valor)
    .max()
    .unwrap();
  
  let max_credito = transacoes.iter()
    .filter(|x| x.tipo == "credito")
    .map(|x| x.valor)
    .max()
    .unwrap();
  
  let saldo_final = v_credito - v_debito;
  let total_transacoes = qtd_credito + qtd_debito;
  
  // Para converter valor monetário do tipo inteiro para float
  // let valor_f64 = valor_int as f64 / 100.0;
  
  println!("=== RELATÓRIO DE TRANSAÇÕES ===");
  println!("Total de transações: {} (crédito: {}, débito: {}", total_transacoes, qtd_credito, qtd_debito);
  println!("Saldo final: R$ {:.2}", saldo_final as f64 / 100.0);
  println!("Maior crédito: R$ {:.2} (Salário)", max_credito as f64 / 100.0);
  println!("Maior débito: R$ {:.2} (Restaurante)", max_debito as f64 / 100.0);
  println!("Erros encotrados");
  
  for e in transacoes.iter() {
    if e.tipo == "Erro" {
      println!("{}", e.descricao);
    }
  }
  
  Ok(())
}

fn main() -> Result<(), ErroTransacao> {
  let arquivo = "transacoes.csv";
  ler_transacoes(arquivo).map_err(|_| ErroTransacao:: ArquivoNaoEncontrado(arquivo.to_string()))?;
  Ok(())
}
