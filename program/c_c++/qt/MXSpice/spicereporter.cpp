#include <QtWidgets>
#include "spicereporter.h"
#include "customerevent.h"

SpiceReporter::SpiceReporter(QObject *parent)
{
    m_parent = parent;
}

void SpiceReporter::OnStateChange(bool exited)
{
    const char *state;
    if (exited) {
        state = "exited";
    } else {
        state = "running";
    }
    CustomerEvent event(state);
    auto ret = QCoreApplication::sendEvent(m_parent, &event);
    qDebug() << "sent ret: " << ret;
}
