+++
title = "{{ post_title }}"
{% if post_excerpt -%}
description = "{{ post_excerpt }}"
{% endif -%}

date = {{ post_date }}
updated = {{ post_modified }}
+++
{{ post_content | markdown }}
