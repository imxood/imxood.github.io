#include "customerevent.h"

QEvent::Type CustomerEvent::m_eventType = QEvent::None;

CustomerEvent::CustomerEvent(QString value) : QEvent(eventType())
{
    m_string = value;
}

QEvent::Type CustomerEvent::eventType()
{
    if (m_eventType == QEvent::None) {
        m_eventType = (QEvent::Type)QEvent::registerEventType();
    }
    return m_eventType;
}

QString CustomerEvent::getValueString()
{
    return m_string;
}
