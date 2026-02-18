//! Insights Slide Plugin
//!
//! Generates automated insights and recommendations based on report data.
//! Analyzes patterns and suggests actionable next steps.

use crate::plugins::{PluginContext, SlideOutput, SlidePlugin};

/// Plugin that generates the Insights & Recommendations slide
pub struct InsightsSlidePlugin;

impl SlidePlugin for InsightsSlidePlugin {
    fn id(&self) -> &'static str {
        "builtin.insights"
    }

    fn name(&self) -> &'static str {
        "Insights & Recommendations"
    }

    fn priority(&self) -> i32 {
        25 // Before closing, after all data slides
    }

    fn is_enabled(&self, ctx: &PluginContext) -> bool {
        ctx.data.total_tickets >= 5 && ctx.config.is_plugin_enabled(self.id())
    }

    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        let data = ctx.data;
        let t = ctx.translations;

        // Generate insights based on data patterns
        let insights = generate_insights(data);

        let title = t.get("insights_title");

        // Build insight cards HTML
        let insight_cards: String = insights
            .iter()
            .map(render_insight_card)
            .collect::<Vec<_>>()
            .join("\n");

        let html = format!(
            r##"<div class="relative group"><div class="printable-slide aspect-[16/9] w-full flex flex-col p-10 md:p-14 shadow-lg mb-8 relative bg-black text-white">
<div class="flex-grow h-full overflow-hidden">
<div class="h-full flex flex-col">
  <!-- Header -->
  <div class="mb-4">
    <span class="bg-[#FF671F] text-white px-4 py-2 text-sm font-bold tracking-wider uppercase">RECOMENDACIONES</span>
  </div>
  <h2 class="text-4xl font-black mb-6 uppercase tracking-tight">{title}</h2>
  
  <!-- Insights Grid -->
  <div class="flex-grow grid grid-cols-2 gap-4 overflow-auto">
    {cards}
  </div>
  
  <!-- Summary Footer -->
  <div class="mt-4 bg-zinc-900/30 p-4 rounded-lg border border-zinc-800">
    <p class="text-zinc-400 text-sm">
      <span class="text-[#FF671F] font-bold">💡 Próximos Pasos:</span> 
      {summary}
    </p>
  </div>
</div>
</div>
{footer}
</div></div>"##,
            title = if title.is_empty() {
                "Insights & Recomendaciones".to_string()
            } else {
                title
            },
            cards = insight_cards,
            summary = generate_action_summary(&insights),
            footer = Self::render_footer(t.get("footer_text")),
        );

        vec![SlideOutput {
            id: "insights".into(),
            html,
        }]
    }
}

impl InsightsSlidePlugin {
    fn render_footer(footer_text: String) -> String {
        format!(
            r##"<footer class="absolute bottom-8 left-14 right-14 flex justify-between items-center">
<div class="flex items-center font-black tracking-wider select-none text-white h-5">
  <span class="text-[#FF671F] text-2xl -mr-1">///</span>
  <span class="text-xl">AXUR</span>
</div>
<div class="flex items-center text-xs text-zinc-500">
  <span>{}</span>
</div>
</footer>"##,
            footer_text
        )
    }
}

/// Insight with priority and recommendation
struct Insight {
    icon: &'static str,
    title: String,
    description: String,
    priority: Priority,
    action: String,
}

#[derive(Clone, Copy)]
enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl Priority {
    fn color(&self) -> &'static str {
        match self {
            Priority::Critical => "#EF4444",
            Priority::High => "#F59E0B",
            Priority::Medium => "#3B82F6",
            Priority::Low => "#22C55E",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Priority::Critical => "CRÍTICO",
            Priority::High => "ALTO",
            Priority::Medium => "MEDIO",
            Priority::Low => "BAJO",
        }
    }
}

/// Generate insights from report data
fn generate_insights(data: &crate::api::report::PocReportData) -> Vec<Insight> {
    let mut insights = Vec::new();

    // Insight 1: High credential exposure
    if data.credentials_total > 100 {
        insights.push(Insight {
            icon: "🔑",
            title: "Exposición de Credenciales Elevada".to_string(),
            description: format!(
                "{} credenciales detectadas. Riesgo de account takeover.",
                data.credentials_total
            ),
            priority: if data.credentials_total > 500 {
                Priority::Critical
            } else {
                Priority::High
            },
            action: "Activar MFA y revisar políticas de contraseñas".to_string(),
        });
    }

    // Insight 2: Low takedown rate
    let total_takedowns = data.takedown_resolved
        + data.takedown_pending
        + data.takedown_aborted
        + data.takedown_unresolved;
    let takedown_rate = if total_takedowns > 0 {
        (data.takedown_resolved as f64 / total_takedowns as f64) * 100.0
    } else {
        0.0
    };

    if takedown_rate < 50.0 && total_takedowns > 20 {
        insights.push(Insight {
            icon: "⚡",
            title: "Tasa de Takedown Baja".to_string(),
            description: format!(
                "Solo {:.0}% de takedowns exitosos. {} pendientes.",
                takedown_rate,
                total_takedowns - data.takedown_resolved
            ),
            priority: Priority::High,
            action: "Escalar con proveedores y revisar SLAs".to_string(),
        });
    }

    // Insight 3: Code secrets exposed
    if data.secrets_total > 10 {
        insights.push(Insight {
            icon: "📦",
            title: "Secretos en Código Expuestos".to_string(),
            description: format!(
                "{} secretos detectados en repositorios públicos.",
                data.secrets_total
            ),
            priority: if data.production_secrets > 5 {
                Priority::Critical
            } else {
                Priority::Medium
            },
            action: "Rotar credenciales y auditar repositorios".to_string(),
        });
    }

    // Insight 4: Phishing campaigns
    let phishing_count: u64 = data
        .threats_by_type
        .iter()
        .filter(|t| t.threat_type.to_lowercase().contains("phishing"))
        .map(|t| t.count)
        .sum();

    if phishing_count > 5 {
        insights.push(Insight {
            icon: "🎣",
            title: "Campañas de Phishing Activas".to_string(),
            description: format!(
                "{} sitios de phishing detectados contra la marca.",
                phishing_count
            ),
            priority: if phishing_count > 20 {
                Priority::Critical
            } else {
                Priority::High
            },
            action: "Alertar a clientes y reforzar awareness".to_string(),
        });
    }

    // Insight 5: Social media fake profiles
    let social_count: u64 = data
        .threats_by_type
        .iter()
        .filter(|t| {
            t.threat_type.to_lowercase().contains("social")
                || t.threat_type.to_lowercase().contains("fake")
        })
        .map(|t| t.count)
        .sum();

    if social_count > 3 {
        insights.push(Insight {
            icon: "👤",
            title: "Suplantación en Redes Sociales".to_string(),
            description: format!("{} perfiles falsos identificados.", social_count),
            priority: Priority::Medium,
            action: "Reportar a plataformas y monitorear VIPs".to_string(),
        });
    }

    // Insight 6: Good efficiency (positive)
    if takedown_rate > 80.0 && data.takedown_resolved > 10 {
        insights.push(Insight {
            icon: "✅",
            title: "Excelente Tasa de Resolución".to_string(),
            description: format!(
                "{:.0}% de takedowns exitosos. Tiempo de respuesta efectivo.",
                takedown_rate
            ),
            priority: Priority::Low,
            action: "Mantener el proceso actual y documentar".to_string(),
        });
    }

    // Insight 7: High threat volume
    if data.total_tickets > 500 {
        insights.push(Insight {
            icon: "📈",
            title: "Volumen de Amenazas Elevado".to_string(),
            description: format!(
                "{} detecciones totales. Considerar recursos adicionales.",
                data.total_tickets
            ),
            priority: Priority::Medium,
            action: "Evaluar automatización y priorización".to_string(),
        });
    }

    // If no specific insights, add general one
    if insights.is_empty() {
        insights.push(Insight {
            icon: "📊",
            title: "Postura de Seguridad Estable".to_string(),
            description: "No se detectaron anomalías críticas en el período.".to_string(),
            priority: Priority::Low,
            action: "Continuar monitoreo proactivo".to_string(),
        });
    }

    // Sort by priority
    insights.sort_by_key(|i| match i.priority {
        Priority::Critical => 0,
        Priority::High => 1,
        Priority::Medium => 2,
        Priority::Low => 3,
    });

    // Limit to 6 insights
    insights.truncate(6);

    insights
}

fn render_insight_card(insight: &Insight) -> String {
    format!(
        r##"<div class="bg-zinc-900/60 p-4 rounded-lg border border-zinc-800">
  <div class="flex items-start gap-3">
    <span class="text-2xl">{icon}</span>
    <div class="flex-grow">
      <div class="flex items-center justify-between mb-1">
        <h3 class="text-sm font-bold text-white">{title}</h3>
        <span class="text-xs font-bold px-2 py-0.5 rounded" style="background: {color}20; color: {color}">{priority}</span>
      </div>
      <p class="text-xs text-zinc-400 mb-2">{desc}</p>
      <div class="flex items-center gap-1 text-xs">
        <span class="text-[#FF671F]">→</span>
        <span class="text-zinc-300">{action}</span>
      </div>
    </div>
  </div>
</div>"##,
        icon = insight.icon,
        title = insight.title,
        color = insight.priority.color(),
        priority = insight.priority.label(),
        desc = insight.description,
        action = insight.action,
    )
}

fn generate_action_summary(insights: &[Insight]) -> String {
    let critical_count = insights
        .iter()
        .filter(|i| matches!(i.priority, Priority::Critical))
        .count();
    let high_count = insights
        .iter()
        .filter(|i| matches!(i.priority, Priority::High))
        .count();

    if critical_count > 0 {
        format!(
            "Se identificaron {} issues críticos que requieren atención inmediata.",
            critical_count
        )
    } else if high_count > 0 {
        format!(
            "{} áreas de mejora prioritarias identificadas. Revisar plan de acción.",
            high_count
        )
    } else {
        "La postura de seguridad es estable. Continuar con monitoreo proactivo.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_colors() {
        assert_eq!(Priority::Critical.color(), "#EF4444");
        assert_eq!(Priority::Low.color(), "#22C55E");
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = InsightsSlidePlugin;
        assert_eq!(plugin.id(), "builtin.insights");
        assert_eq!(plugin.priority(), 25);
    }
}
