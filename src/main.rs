struct RegistroVenda {
    mes: f64,
    valor: f64,
}

fn calc_media(valores: &[f64]) -> f64 {
    let soma: f64 = valores.iter().sum();
    soma / valores.len() as f64
}

fn calc_inclinacao(dados: &[RegistroVenda]) -> f64 {
    let meses: Vec<f64> = dados.iter().map(|d| d.mes).collect();
    let valores: Vec<f64> = dados.iter().map(|d| d.valor).collect();

    let media_meses = calc_media(&meses);
    let media_valores = calc_media(&valores);

    let numerador: f64 = dados
        .iter()
        .map(|d| (d.mes - media_meses) * (d.valor - media_valores))
        .sum();

    let denominador: f64 = dados.iter().map(|d| (d.mes - media_meses).powi(2)).sum();

    numerador / denominador
}

fn calc_intercepto(dados: &[RegistroVenda], inclinacao: f64) -> f64 {
    let media_meses = calc_media(&dados.iter().map(|d| d.mes).collect::<Vec<f64>>());
    let media_valores = calc_media(&dados.iter().map(|d| d.valor).collect::<Vec<f64>>());
    media_valores - (inclinacao * media_meses)
}

fn prever_valor(intercepto: f64, inclinacao: f64, mes: f64) -> f64 {
    intercepto + (inclinacao * mes)
}

fn calc_mse(dados: &[RegistroVenda], inclinacao: f64, intercepto: f64) -> f64 {
    let erro_quadratico: f64 = dados
        .iter()
        .map(|d| {
            let previsto = prever_valor(intercepto, inclinacao, d.mes);
            (d.valor - previsto).powi(2)
        })
        .sum();
    erro_quadratico / dados.len() as f64
}

fn calc_r2(dados: &[RegistroVenda], inclinacao: f64, intercepto: f64) -> f64 {
    let media_valores = calc_media(&dados.iter().map(|d| d.valor).collect::<Vec<f64>>());
    let ss_total: f64 = dados
        .iter()
        .map(|d| (d.valor - media_valores).powi(2))
        .sum();
    let ss_res: f64 = dados
        .iter()
        .map(|d| {
            let previsto = prever_valor(intercepto, inclinacao, d.mes);
            (d.valor - previsto).powi(2)
        })
        .sum();
    1.0 - (ss_res / ss_total)
}

fn main() {
    let dados_vendas = vec![
        RegistroVenda { 
            mes: 1.0, 
            valor: 90.0 
        },
        RegistroVenda { 
            mes: 2.0, 
            valor: 117.5 
        },
        RegistroVenda { 
            mes: 3.0, 
            valor: 145.0 
        },
    ];

    let inclinacao = calc_inclinacao(&dados_vendas);
    let intercepto = calc_intercepto(&dados_vendas, inclinacao);
    let previsao = prever_valor(intercepto, inclinacao, 6.0);

    let mse = calc_mse(&dados_vendas, inclinacao, intercepto);
    let r2 = calc_r2(&dados_vendas, inclinacao, intercepto);

    println!(
        "Inclinação: {:.2}, Intercepto: {:.2}",
        inclinacao, intercepto
    );
    println!("Previsão para mês 6: {:.2}", previsao);
    println!("MSE: {:.2}, R²: {:.2}", mse, r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_media() {
        let valores = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let resultado = calc_media(&valores);
        assert_eq!(resultado, 3.0);
    }

    #[test]
    fn test_calc_media_iguais() {
        let valores = vec![9.0, 9.0, 9.0, 9.0, 9.0];
        let resultado = calc_media(&valores);
        assert_eq!(resultado, 9.0);
    }

    #[test]
    fn test_calc_media_unico_valor() {
        let valores = vec![5.0];
        let resultado = calc_media(&valores);
        assert_eq!(resultado, 5.0);
    }

    #[test]
    fn test_calc_media_negativos() {
        let valores = vec![-2.0, -4.0, -6.0];
        let resultado = calc_media(&valores);
        assert_eq!(resultado, -4.0);
    }

    #[test]
    fn test_calc_media_negativos_positivos() {
        let valores = vec![-2.0, 4.0];
        let resultado = calc_media(&valores);
        assert_eq!(resultado, 1.0);
    }

    #[test]
    fn test_prever_valor() {
        let intercepto = 10.0;
        let inclinacao = 5.0;
        let mes = 2.0;
        let resultado = prever_valor(intercepto, inclinacao, mes);
        assert_eq!(resultado, 20.0);
    }

    #[test]
    fn test_mse_e_r2_simples() {
        let dados = vec![
            RegistroVenda { mes: 1.0, valor: 2.0 },
            RegistroVenda { mes: 2.0, valor: 4.0 },
            RegistroVenda { mes: 3.0, valor: 6.0 },
        ];
        let inclinacao = calc_inclinacao(&dados);
        let intercepto = calc_intercepto(&dados, inclinacao);
        let mse = calc_mse(&dados, inclinacao, intercepto);
        let r2 = calc_r2(&dados, inclinacao, intercepto);

        assert!(mse < 1e-6); 
        assert!((r2 - 1.0).abs() < 1e-6); 
    }

    #[test]
    fn test_calc_inclinacao_simples() {
        let dados = vec![
            RegistroVenda { mes: 1.0, valor: 2.0 },
            RegistroVenda { mes: 2.0, valor: 4.0 },
            RegistroVenda { mes: 3.0, valor: 6.0 },
        ];
        let resultado = calc_inclinacao(&dados);
        let esperado = 2.0;
        assert!(
            (resultado - esperado).abs() < 1e-6,
            "Inclinação incorreta: esperada {}, obtida {}",
            esperado,
            resultado
        );
    }

    #[test]
    fn test_calc_intercepto_simples() {
        let dados = vec![
            RegistroVenda { mes: 1.0, valor: 3.0 },
            RegistroVenda { mes: 2.0, valor: 5.0 },
            RegistroVenda { mes: 3.0, valor: 7.0 },
        ];
        let inclinacao = calc_inclinacao(&dados);
        let resultado = calc_intercepto(&dados, inclinacao);
        let esperado = 1.0; 
        assert!(
            (resultado - esperado).abs() < 1e-6,
            "Intercepto incorreto: esperado {}, obtido {}",
            esperado,
            resultado
        );
    }
}
